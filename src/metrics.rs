use crate::shared_types::MetricsConfig;
extern crate globwalk;

use std::fs;
use std::path::Path;
use rust_code_analysis::{LANG, ParserTrait, RustParser};
use self::globwalk::DirEntry;

pub fn execute(config: MetricsConfig) {
    println!("Path: {}", config.repository_path);
    println!("Verbosity: {}", config.verbosity);
    let base_dir = Path::new(&config.repository_path);

    let walker = globwalk::GlobWalkerBuilder::from_patterns(base_dir, &["*.{cs,c,cpp,fs,go,js,java,py,rs,ts}", "!.*", "!**target/*"])
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok);

    let mut files_scanned = 0;
    let mut files_walked = 0;

    let timer = eggtimer::Timer::start();

    for de in walker {
        let dir_entry: DirEntry = de;
        let path = dir_entry.path();
        let lang = rust_code_analysis::get_from_ext(path.extension().unwrap().to_str().unwrap());
        match lang {
            Some(LANG::Rust) => {
                let contentsOpt = fs::read(path);
                if contentsOpt.is_ok() {
                    let contents = fs::read(path).unwrap();
                    let ext = path.extension().unwrap().to_str().unwrap();
                    let path_buf = path.to_path_buf();
                    let parser = RustParser::new(contents, &path_buf, None);
                    let metrics = rust_code_analysis::metrics(&parser, &path_buf);
                    match metrics {
                        Some(m) => { println!("File: {} | Complexity: {}", path.display(), m.metrics.cyclomatic); },
                        None => ()
                    }
                    files_scanned += 1;
                }
            },
            _ => ()
        }
        files_walked += 1;
    }
    let time_taken_sec = timer.elapsed();
    println!("File scanned: {}", files_scanned);
    println!("File matches: {}", files_walked);
    println!("Time taken: {}s", time_taken_sec);
}