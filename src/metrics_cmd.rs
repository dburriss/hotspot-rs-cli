use crate::shared_types::{MetricsConfig, SpecificMetrics};
extern crate globwalk;

use std::fs;
use std::iter::FilterMap;
use std::path::Path;
use rust_code_analysis::{LANG, ParserTrait, RustParser};
use self::globwalk::{DirEntry, GlobWalker, WalkError};

pub fn execute(config: MetricsConfig) {
    if config.verbosity.is_not_quiet() {
        println!("Executing metrics command...");
        println!("Verbosity: {}", config.verbosity);
        println!("Path: {}", config.repository_path);
    }

    let mut files_scanned = 0;
    let mut files_walked = 0;

    let timer = eggtimer::Timer::start();

    let base_dir = Path::new(&config.repository_path);
    let walker = setup_file_walker(base_dir);
    let metrics = walker.filter_map(|de| {
        let dir_entry: DirEntry = de;
        let path = dir_entry.path();
        let m = get_metrics(files_scanned, path);
        files_walked += 1;
        m
    });

    for m in metrics {
        println!("File: {} | LoC: {}", m.path, m.loc.unwrap());
    }

    let time_taken_sec = timer.elapsed();
    if config.verbosity.is_not_quiet() {
        println!("Files scanned for metrics: {}", files_scanned);
        println!("Total files matched: {}", files_walked);
        println!("Metrics command completed in: {}s", time_taken_sec);
    }
}

fn get_metrics(mut files_scanned: i32, path: &Path) -> Option<SpecificMetrics> {

    let ext = path.extension().unwrap().to_str().unwrap();
    let lang = rust_code_analysis::get_from_ext(ext);
    let m = match lang {
        Some(LANG::Rust) => {
            let contentsOpt = fs::read(path);
            if contentsOpt.is_ok() {
                let contents = fs::read(path).unwrap();
                let path_buf = path.to_path_buf();
                let parser = RustParser::new(contents, &path_buf, None);
                let metrics = rust_code_analysis::metrics(&parser, &path_buf);
                let p = path.to_str().map(|s| { String::from(s) }).unwrap();
                files_scanned += 1;
                let m = SpecificMetrics {
                    path: p,
                    loc: metrics.map(|x| x.metrics.loc.sloc() as i64),
                    cognitive: None,
                    complexity: None
                };
                Some(m)
            } else { None }
        },
        _ => None
    };
    m
}

fn setup_file_walker(base_dir: &Path) -> FilterMap<GlobWalker, fn(Result<DirEntry, WalkError>) -> Option<DirEntry>> {
    globwalk::GlobWalkerBuilder::from_patterns(base_dir, &["*.{cs,c,cpp,fs,go,js,java,py,rs,ts,tsx}", "!.*", "!**target/*"])
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
}