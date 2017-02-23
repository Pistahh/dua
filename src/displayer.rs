
use du::{Entry,EntryData};

#[derive(PartialEq,Eq)]
pub enum DisplayType {
    Normal,
    HumanReadable,
    KiBytes,
}

pub struct Displayer {
    dt: DisplayType,
    num: usize,
    depth: u32,
}

impl Displayer {
    pub fn new(dt: DisplayType, num: usize, depth: u32) -> Displayer {
        Displayer { dt: dt, num: num, depth: depth }
    }

    pub fn display(&self, entry: &Entry) {
        self.show_usage_level(entry, 0, entry.size()) ;
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

        let Entry(ref name, ref data) = *entry;

        let size = entry.size();

        let pct = if allsize == 0 { 0.0 } else { size as f64 / allsize as f64 * 100.0 };

        print!("{:>5.1}% ", pct);

        match self.dt {
            DisplayType::Normal        => { print!("{:12}", size); }
            DisplayType::KiBytes       => { print!("{:9}", size/1024); }
            DisplayType::HumanReadable => { print!("{:6}", self.to_human(size)); }
        }

        print!(" {}", name.display());
        match data {
            &EntryData::Directory{..} => { print!("/"); },
            &EntryData::OtherFs      => { print!(" [X]"); },
            &EntryData::Other        => { print!(" [?]"); },
            &EntryData::Error(ref error) => {
                print!(" [ERROR: ");
                if let Some(ref inner) = error.get_ref() {
                    print!("{:?}", inner);
                } else {
                    print!("{:?}", error.kind())
                };
                print!("]");
            },
            _ => {}
        };
        println!();

    }

    fn show_usage_level(&self, entry: &Entry, level: u32, allsize: u64) {
        self.show_entry(level, &entry, allsize);
        if level < self.depth-1 {
            if let Entry(_, EntryData::Directory{ size, ref children }) = *entry {
                let mut n = self.num;
                for ce in children {
                    self.show_usage_level(&ce, level+1, size);
                    n = n-1;
                    if n == 0 { break; }
                }
            }
        }
    }
}

