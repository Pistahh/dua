
use std::fs::{read_dir, symlink_metadata};
use std::path::{PathBuf, Path};
use std::io;
use std::env;
use std::os::unix::fs::MetadataExt;

#[derive(Debug)]
pub enum EntryData {
    File { size: u64 },
    Directory { size: u64, children: Vec<Entry> },
    OtherFs,
    Error(io::Error),
    Other,
}

#[derive(Debug)]
pub struct Entry(pub PathBuf, pub EntryData);

impl Entry {
    pub fn size(&self) -> u64 {
        let Entry(_, ref data) = *self;

        match data {
            &EntryData::File{size}  => size,
            &EntryData::Directory{size, ..} => size,
            _ => 0
        }
    }
}

pub fn process_entry(name: &Path, xfs: bool, dev: Option<u64>) -> Entry {

    let m = symlink_metadata(name);

    let name = PathBuf::from(name);

    if let Err(err) = m {
        return Entry(name, EntryData::Error(err));
    }

    let m = m.unwrap();

    let mdev = m.dev();

    let dev = dev.unwrap_or(if xfs { mdev } else { 0 });

    if xfs && mdev != dev {
        return Entry(name, EntryData::OtherFs);
    }

    if m.is_file() {
        return Entry(name, EntryData::File { size: m.len() });

    } else if m.is_dir() {
        let cwd = env::current_dir().unwrap();

        if let Err(err) = env::set_current_dir(&name) {
            return Entry(name, EntryData::Error(err));
        }

        let dir_list = read_dir(".");

        if let Err(err) = dir_list {
            env::set_current_dir(&cwd).unwrap();
            return Entry(name, EntryData::Error(err));
        }

        let mut v: Vec<Entry> = vec![];

        for entry in dir_list.unwrap() {
            let entry=entry.unwrap();
            let subentry = process_entry(entry.file_name().as_ref(),
                                         xfs,
                                         Some(dev));
            v.push(subentry);
        }

        assert!(env::set_current_dir(cwd).is_ok());

        let total_size = v.iter().map(|x| x.size()).sum();
        v.sort_by(|a,b| b.size().cmp(&a.size()));

        return Entry(name, EntryData::Directory {
                                size: total_size,
                                children: v });
    } else {
        return Entry(name, EntryData::Other);
    }
}
