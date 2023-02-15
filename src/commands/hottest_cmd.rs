use git2::Repository;
use hotspot::shared_types::{
    is_supported_file, truncate_left, truncate_right, HottestConfig, FILE_GLOBS,
};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use term_table::TableStyle;

extern crate chrono;
use chrono::prelude::*;

struct HottestReport {
    touches: u32,
    path: String,
    created_at: i64,
    created_by: String,
    last_touched_at: i64,
    last_touched_by: String,
}

pub fn execute(config: HottestConfig) {
    if config.verbosity.is_not_quiet() {
        println!("Executing hottest command...");
        println!("Verbosity: {}", config.verbosity);
        println!("Path: {}", config.repository_path);
    }

    let base_dir = Path::new(&config.repository_path);
    let repo = match Repository::open(base_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let mut file_touches: HashMap<String, HottestReport> = HashMap::new();
    let mut rev_walk = repo.revwalk().unwrap();
    rev_walk.push_head().unwrap();
    let mut i = 0;
    for elem in rev_walk {
        let oid = elem.unwrap();
        let commit = repo.find_commit(oid).unwrap();

        let author = commit.author();
        let name = author.name().unwrap();
        let email = author.email().unwrap();
        let identifier = format!("{}<{}>", name, email);
        let parent_count = commit.parent_count();
        if parent_count == 0 || parent_count == 1 {
            let tree = commit.tree().unwrap();
            let diff = if parent_count == 0 {
                let prev_tree = None;
                let diff = repo
                    .diff_tree_to_tree(prev_tree, Some(&tree), None)
                    .unwrap();
                diff
            } else if parent_count == 1 {
                let prev_commit = commit.parent(0).unwrap();
                let prev_tree = prev_commit.tree().unwrap();
                let diff = repo
                    .diff_tree_to_tree(Some(&prev_tree), Some(&tree), None)
                    .unwrap();
                diff
            } else {
                panic!("`parent_count` unexpectedly {}", parent_count);
            };

            for delta in diff.deltas() {
                let file_path = delta.new_file().path().unwrap();
                let file_mod_time = commit.time();
                let unix_time = file_mod_time.seconds();
                let path_str = file_path.to_str().unwrap();
                let path = path_str.to_string();

                if is_supported_file(FILE_GLOBS.to_vec(), path_str) {
                    file_touches
                        .entry(path.clone())
                        .and_modify(|e| {
                            e.touches += 1;
                        })
                        .or_insert(HottestReport {
                            touches: 1,
                            path,
                            created_by: identifier.to_string(),
                            created_at: unix_time,
                            last_touched_by: identifier.to_string(),
                            last_touched_at: unix_time,
                        });
                }
            }
        }

        i = i + 1;
    }

    output(config, file_touches);
}

fn output(_config: HottestConfig, file_touches: HashMap<String, HottestReport>) {
    let mut table = term_table::Table::new();
    table.max_column_width = 400;
    table.style = TableStyle::thin();
    table.add_row(term_table::row::Row::new(vec![
        term_table::table_cell::TableCell::new("Path"),
        term_table::table_cell::TableCell::new("Changes"),
        term_table::table_cell::TableCell::new("Last changed by"),
        term_table::table_cell::TableCell::new("Last changed at"),
    ]));
    let mut file_touch_vec = file_touches
        .iter()
        .map(|(path, row)| row)
        .collect::<Vec<&HottestReport>>();
    file_touch_vec.sort_by_key(|k| k.touches);
    file_touch_vec.reverse(); // TODO: implement cmp on HottestReport and use that instead
    if _config.top > 0 {
        file_touch_vec.truncate(_config.top as usize);
    }
    for row_data in file_touch_vec {
        table.add_row(term_table::row::Row::new(vec![
            term_table::table_cell::TableCell::new(truncate_left(row_data.path.to_string(), 70)),
            term_table::table_cell::TableCell::new_with_alignment(
                row_data.touches,
                1,
                term_table::table_cell::Alignment::Right,
                //
            ),
            term_table::table_cell::TableCell::new(truncate_right(
                row_data.last_touched_by.to_string(),
                70,
            )),
            term_table::table_cell::TableCell::new(truncate_right(
                NaiveDateTime::from_timestamp(row_data.last_touched_at, 0).to_string(),
                70,
            )),
        ]));
    }
    println!("{}", table.render());
}
