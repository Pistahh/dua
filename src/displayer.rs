
use du::{Entry,EntryType};

#[derive(PartialEq,Eq)]
pub enum DisplayType {
    Normal,
    HumanReadable,
    KiBytes,
}

pub struct Displayer {
    dt: DisplayType,
    num: usize,
}

impl Displayer {
    pub fn new(dt: DisplayType, num: usize) -> Displayer {
        Displayer { dt: dt, num: num }
    }

    pub fn display(&self, entry: &Entry) {
        self.show_usage_level(entry, 0, entry.size) ;
    }

    fn to_human(&self, size: u64) -> String {

        let size = size as f64;
        let k=1024.0;
        if size < k {
            format!("{:-6}", size)
        } else if size < k*k {
            format!("{:-5.1}K", size/k)
        } else if size < k*k*k {
            format!("{:-5.1}M", size/k/k)
        } else if size < k*k*k*k {
            format!("{:-5.1}G", size/k/k/k)
        } else if size < k*k*k*k*k {
            format!("{:-5.1}T", size/k/k/k/k)
        } else if size < k*k*k*k*k*k {
            format!("{:-5.1}P", size/k/k/k/k/k)
        } else {
            format!("{:-5.1}E", size/k/k/k/k/k/k)
        }
    }

    fn show_entry(&self, level: u32, entry: &Entry, allsize: u64) {
            //println!("{:?}", entry);
            for _ in 0..level {
                print!("   ");
            }

            let pct = if allsize == 0 { 0.0 } else { entry.size as f64 / allsize as f64 * 100.0 };

            print!("{:>5.1}% ", pct);

            match self.dt {
                DisplayType::Normal        => { print!("{:12}", entry.size); }
                DisplayType::KiBytes       => { print!("{:9}", entry.size/1024); }
                DisplayType::HumanReadable => { print!("{:6}", self.to_human(entry.size)); }
            }

            print!(" {}", entry.name.display());
            match entry.entrytype {
                EntryType::Directory => { print!("/"); },
                EntryType::OtherFs   => { print!(" [X]"); },
                EntryType::Other     => { print!(" [?]"); },
                _ => {}
            };
            println!();

    }

    fn show_usage_level(&self, entry: &Entry, level: u32, allsize: u64) {
            self.show_entry(level, &entry, allsize);
            if entry.entrytype == EntryType::Directory {
                    let mut n = self.num;
                    for ce in &entry.children {
                        self.show_usage_level(&ce, level+1, entry.size);
                        n = n-1;
                        if n == 0 { break; }
                    }
            }
    }
}

