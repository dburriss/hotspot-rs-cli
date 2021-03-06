use git2::Repository;
use hotspot::shared_types::{
    is_supported_file, truncate_left, BusFactorConfig, ContributorKey, FILE_GLOBS,
};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use term_table::TableStyle;

pub fn execute(config: BusFactorConfig) {
    if config.verbosity.is_not_quiet() {
        println!("Executing busfactor command...");
        println!("Verbosity: {}", config.verbosity);
        println!("Path: {}", config.repository_path);
    }

    let base_dir = Path::new(&config.repository_path);
    let repo = match Repository::open(base_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let mut file_contributors: HashMap<String, HashSet<ContributorKey>> = HashMap::new();
    let mut rev_walk = repo.revwalk().unwrap();
    rev_walk.push_head().unwrap();
    let mut i = 0;
    for elem in rev_walk {
        let oid = elem.unwrap();
        let commit = repo.find_commit(oid).unwrap();

        let author = commit.author();
        let name = author.name().unwrap();
        let email = author.email().unwrap();
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
                //let file_mod_time = commit.time();
                //let unix_time = file_mod_time.seconds();
                let path_str = file_path.to_str().unwrap();
                let key = path_str.to_string();

                if is_supported_file(FILE_GLOBS.to_vec(), path_str) {
                    let h = file_contributors
                        .entry(key.clone())
                        .or_insert(HashSet::new());
                    h.insert(ContributorKey::new(email.to_string(), name.to_string()));
                }
            }
        }

        i = i + 1;
    }

    output(config, file_contributors, i);
}

fn output(
    _config: BusFactorConfig,
    file_contributors: HashMap<String, HashSet<ContributorKey>>,
    _commit_count: i32,
) {
    let mut table = term_table::Table::new();
    table.max_column_width = 400;
    table.style = TableStyle::thin();
    table.add_row(term_table::row::Row::new(vec![
        term_table::table_cell::TableCell::new("Path"),
        term_table::table_cell::TableCell::new("Bus factor"),
    ]));
    for (p, cs) in file_contributors {
        table.add_row(term_table::row::Row::new(vec![
            term_table::table_cell::TableCell::new(truncate_left(p, 70)),
            term_table::table_cell::TableCell::new_with_alignment(
                cs.len(),
                1,
                term_table::table_cell::Alignment::Right,
            ),
        ]));
    }
    println!("{}", table.render());
}
