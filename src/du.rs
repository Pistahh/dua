
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
    Other,
}

#[derive(Debug)]
pub struct Entry {
    pub name: PathBuf,
    pub size: u64,
    pub entrytype: EntryType,
    pub children: Option<Vec<Entry>>
}

impl Entry {
    fn new(name: &Path, size: u64, entrytype: EntryType, children: Option<Vec<Entry>>) -> Entry {
        Entry { name: PathBuf::from(name),
            size: size,
            entrytype: entrytype,
            children: children
        }
    }
}

pub fn process_entry(name: &Path, xfs: bool, dev: Option<u64>) -> io::Result<Entry> {
    let m = try!(symlink_metadata(name));
  
    let mdev = m.dev();

    let dev = dev.unwrap_or(if xfs { mdev } else { 0 });

    if xfs && mdev != dev {
        return Ok(Entry::new(name,
                    0,
                    EntryType::OtherFs, 
                    None));
    }

    if m.is_file() {
        return Ok(Entry::new(name,
                    m.len(),
                    EntryType::File, 
                    None));
    } else if m.is_dir() {
        let mut v: Vec<Entry> = vec![];
        let cwd = try!(env::current_dir());
        try!(env::set_current_dir(name));
        for entry in try!(read_dir(".")) {
            let entry=try!(entry);
            let subentry = try!(process_entry(entry.file_name().as_ref(),
                                              xfs,
                                              Some(dev)));
            v.push(subentry);
        }
        assert!(env::set_current_dir(cwd).is_ok());

        let total_size = v.iter().map(|x| x.size).sum();
        v.sort_by(|a,b| b.size.cmp(&a.size));

        return Ok(Entry::new(name,
                    total_size,
                    EntryType::Directory,
                    Some(v)));
    } else {
        return Ok(Entry::new(name,
                    0,
                    EntryType::Other,
                    None));
    }
}
