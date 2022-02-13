use crate::shared_types::{truncate_left, MetricsConfig, SpecificMetrics};
extern crate globwalk;

use self::globwalk::{DirEntry, GlobWalker, WalkError};
use rust_code_analysis::{
    CppParser, FuncSpace, JavascriptParser, ParserTrait, PreprocParser, PythonParser, RustParser,
    TsxParser, TypescriptParser, LANG,
};
use std::fs;
use std::iter::FilterMap;
use std::path::{Path, PathBuf};

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
        let m = get_metrics(path);
        if m.is_some() {
            files_scanned += 1;
        }
        files_walked += 1;
        m
    });
    let time_taken_sec = timer.elapsed();

    println!("+-{:-<80}---{:-<6}---{:-<9}---{:-<10}-+", "", "", "", "");
    println!(
        "| {: <80} | {:6} | {:9} | {:10} |",
        "File", "LoC", "Cognitive", "Cyclomatic"
    );
    println!("|={:=<80}==={:=<6}==={:=<9}==={:=<10}=|", "", "", "", "");
    for m in metrics {
        if m.loc.is_some() {
            let loc = m.loc.map(|x| x.to_string()).unwrap_or(String::new());
            let cog = m.cognitive.map(|x| x.to_string()).unwrap_or(String::new());
            let cyc = m.cyclomatic.map(|x| x.to_string()).unwrap_or(String::new());
            println!(
                "| {:<80} | {:<6} | {:<9} | {:<10} |",
                truncate_left(m.path, 80),
                loc,
                cog,
                cyc
            );
        } else if config.verbosity.is_verbose() {
            println!("Skipped: {}", m.path);
        }
    }
    println!("--{:-<80}---{:-<6}---{:-<9}---{:-<10}--", "", "", "", "");

    if config.verbosity.is_not_quiet() {
        println!("Files scanned for metrics: {}", files_scanned);
        println!("Total files matched: {}", files_walked);
        println!("Metrics command completed in: {}s", time_taken_sec);
    }
}

fn get_metrics(path: &Path) -> Option<SpecificMetrics> {
    let contents_opt = fs::read(path);
    if contents_opt.is_ok() {
        let contents = contents_opt.unwrap();
        let path_buf = path.to_path_buf();
        let metrics = get_function_space(contents, &path_buf);
        let p = path.to_str().map(|s| String::from(s)).unwrap();
        let m = match metrics {
            Some(function_space) => SpecificMetrics {
                path: p,
                loc: Some(function_space.metrics.loc.lloc() as i64),
                cognitive: Some(function_space.metrics.cognitive.cognitive() as i64),
                cyclomatic: Some(function_space.metrics.cyclomatic.cyclomatic() as i64),
            },
            None => SpecificMetrics {
                path: p,
                loc: None,
                cognitive: None,
                cyclomatic: None,
            },
        };
        Some(m)
    } else {
        None
    }
}

fn get_function_space(contents: Vec<u8>, path_buf: &PathBuf) -> Option<FuncSpace> {
    let ext = path_buf.extension().unwrap().to_str().unwrap();
    let lang = rust_code_analysis::get_from_ext(ext);
    match lang {
        Some(LANG::Rust) => {
            let parser = RustParser::new(contents, &path_buf, None);
            rust_code_analysis::metrics(&parser, path_buf)
        }
        Some(LANG::Javascript) | Some(LANG::Mozjs) => {
            let parser = JavascriptParser::new(contents, &path_buf, None);
            rust_code_analysis::metrics(&parser, path_buf)
        }
        Some(LANG::Typescript) => {
            let parser = TypescriptParser::new(contents, &path_buf, None);
            rust_code_analysis::metrics(&parser, path_buf)
        }
        Some(LANG::Tsx) => {
            let parser = TsxParser::new(contents, &path_buf, None);
            rust_code_analysis::metrics(&parser, path_buf)
        }
        Some(LANG::Python) => {
            let parser = PythonParser::new(contents, &path_buf, None);
            rust_code_analysis::metrics(&parser, path_buf)
        }
        Some(LANG::Cpp) => {
            let parser = CppParser::new(contents, &path_buf, None);
            rust_code_analysis::metrics(&parser, path_buf)
        }
        Some(LANG::Java) => {
            let parser = PreprocParser::new(contents, &path_buf, None);
            rust_code_analysis::metrics(&parser, path_buf)
        }
        Some(_) => None,
        None => None,
    }
}

fn setup_file_walker(
    base_dir: &Path,
) -> FilterMap<GlobWalker, fn(Result<DirEntry, WalkError>) -> Option<DirEntry>> {
    globwalk::GlobWalkerBuilder::from_patterns(
        base_dir,
        &[
            "*.{cs,c,cpp,fs,go,js,java,py,rs,ts,tsx}",
            "!.*",
            "!node_modules/",
            "!target/",
        ],
    )
    .build()
    .unwrap()
    .into_iter()
    .filter_map(Result::ok)
}
