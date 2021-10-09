use crate::shared_types::MetricsConfig;
extern crate globwalk;

use std::fs;
use std::path::Path;

pub fn execute(config: MetricsConfig) {
    println!("Path: {}", config.repository_path);
    println!("Verbosity: {}", config.verbosity);
    let base_dir = Path::new(&config.repository_path);

    let walker = globwalk::GlobWalkerBuilder::from_patterns(base_dir, &["*.{cs,c,cpp,fs,js,java,rs,ts}", "!target/*"])
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok);

    let mut file_count = 0;
    for file in walker {
        println!("File: {}", file.file_name().to_string_lossy());
        file_count += 1;
    }
    println!("File matches: {}", file_count);

}