
extern crate clap;

use std::path::Path;
use std::process::exit;

use clap::{Arg, App};

mod displayer;
use displayer::{Displayer, DisplayType};

mod du;

fn main() {

    use std::io::Write;

    let mut stderr = std::io::stderr();

    let matches = App::new("Disk Usage Analyzer")
                          .version("1.0")
                          .author("Istvan Szekeres <szekeres@iii.hu>")
                          .about("Analyzes disk usage")
                          .arg(Arg::with_name("xfs")
                               .short("x")
                               .help("no crossing of filesystems")
                               .required(false)
                               .takes_value(false))
                          .arg(Arg::with_name("hr")
                               .short("h")
                               .help("Output sizes in human readable form")
                               .required(false)
                               .takes_value(false))
                          .arg(Arg::with_name("kb")
                               .short("k")
                               .help("Output sizes in KiBytes")
                               .required(false)
                               .takes_value(false))
                          .arg(Arg::with_name("num")
                               .short("n")
                               .help("number of entries to display on each level")
                               .required(false)
                               .takes_value(true))
                          .arg(Arg::with_name("depth")
                               .short("d")
                               .help("depth to display")
                               .required(false)
                               .takes_value(true))
                          .arg(Arg::with_name("DIR")
                               .index(1)
                               .help("Directory to analyze"))
                          .get_matches();

	let dir = matches.value_of("DIR").unwrap_or(".");
        let hr = matches.occurrences_of("hr") > 0;
        let kb = matches.occurrences_of("kb") > 0;
        let xfs = matches.occurrences_of("xfs") > 0;

        if hr && kb {
            writeln!(&mut stderr, "Only one of -k and -h can be used").unwrap();
            exit(1);
        }

        let dm = if hr {
            DisplayType::HumanReadable
        } else if kb {
            DisplayType::KiBytes
        } else {
            DisplayType::Normal
        };

        let num = matches.value_of("num").map_or(1, |x| x.parse::<usize>().unwrap_or(1));
        const DEFAULT_DEPTH: u32 = 999999;
        let depth = matches.value_of("depth").map_or(DEFAULT_DEPTH, |x| x.parse::<u32>().unwrap_or(DEFAULT_DEPTH));

	let res = du::process_entry(Path::new(dir), xfs, None);

	match res {
		Ok(entry)  => {
                    let d = Displayer::new(dm, num, depth);
                    d.display(&entry);
                },
		Err(err) => {
			writeln!(&mut stderr, "{:?}", err.kind()).unwrap();
                        exit(1);
		}
	}
}
