
use std::fs::{read_dir, symlink_metadata};
use std::path::{PathBuf, Path};
use std::io;
use std::env;
use std::os::unix::fs::MetadataExt;

#[derive(Debug, PartialEq, Eq)]
pub enum EntryType {
    File,
    Directory,
    OtherFs,
    Error,
    Other,
}

#[derive(Debug)]
pub struct Entry {
    pub name: PathBuf,
    pub size: u64,
    pub entrytype: EntryType,
    pub children: Option<Vec<Entry>>,
    pub error: Option<io::Error>,
}

impl Entry {
    fn new(name: &Path, size: u64, entrytype: EntryType, children: Option<Vec<Entry>>, error: Option<io::Error>) -> Entry {
        Entry { name: PathBuf::from(name),
            size: size,
            entrytype: entrytype,
            children: children,
            error: error,
        }
    }
}

pub fn process_entry(name: &Path, xfs: bool, dev: Option<u64>) -> Entry {

    let m = symlink_metadata(name);

    if let Err(err) = m {
        return Entry::new(name,
                          0,
                          EntryType::Error,
                          None,
                          Some(err));
    }

    let m = m.unwrap();

    let mdev = m.dev();

    let dev = dev.unwrap_or(if xfs { mdev } else { 0 });

    if xfs && mdev != dev {
        return Entry::new(name,
                          0,
                          EntryType::OtherFs,
                          None,
                          None);
    }

    if m.is_file() {
        return Entry::new(name,
                          m.len(),
                          EntryType::File,
                          None,
                          None);
    } else if m.is_dir() {
        let cwd = env::current_dir().unwrap();
        if let Err(err) = env::set_current_dir(name) {
            return Entry::new(name,
                              0,
                              EntryType::Error,
                              None,
                              Some(err));
        }
        let dir_list = read_dir(".");

        if let Err(err) = dir_list {
            env::set_current_dir(cwd).unwrap();
            return Entry::new(name,
                              0,
                              EntryType::Error,
                              None,
                              Some(err));
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

        let total_size = v.iter().map(|x| x.size).sum();
        v.sort_by(|a,b| b.size.cmp(&a.size));

        return Entry::new(name,
                          total_size,
                          EntryType::Directory,
                          Some(v),
                          None);
    } else {
        return Entry::new(name,
                          0,
                          EntryType::Other,
                          None,
                          None);
    }
}
