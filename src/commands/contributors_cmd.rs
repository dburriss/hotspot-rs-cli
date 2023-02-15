use git2::Repository;
use hotspot::shared_types::{truncate_right, ContributorKey, ContributorsConfig};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use term_table::TableStyle;
// maybe this? https://docs.rs/git2/0.13.22/git2/struct.Repository.html#method.revwalk
// get files see code here: https://github.com/rust-lang/git2-rs/issues/588#issuecomment-856757971
// C# impl https://github.com/libgit2/libgit2sharp/pull/963/files

pub fn execute(config: ContributorsConfig) {
    if config.verbosity.is_not_quiet() {
        println!("Executing contributors command...");
        println!("Verbosity: {}", config.verbosity);
        println!("Path: {}", config.repository_path);
    }
    let base_dir = Path::new(&config.repository_path);
    let repo = match Repository::open(base_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let mut contributors: HashMap<ContributorKey, u32> = HashMap::new();
    let mut contributor_files: HashMap<ContributorKey, HashSet<String>> = HashMap::new();
    let mut rev_walk = repo.revwalk().unwrap();
    rev_walk.push_head().unwrap();
    let mut i = 0;
    for elem in rev_walk {
        let oid = elem.unwrap();
        let commit = repo.find_commit(oid).unwrap();

        let author = commit.author();
        let name = author.name().unwrap();
        let email = author.email().unwrap();
        let key = ContributorKey::new(email.to_string(), name.to_string());
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
                let h = contributor_files
                    .entry(key.clone())
                    .or_insert(HashSet::new());
                h.insert(file_path.to_str().unwrap().to_string());
            }
        }

        i = i + 1;
        *contributors.entry(key.clone()).or_insert(0) += 1;
        let _ = *contributor_files.entry(key).or_insert(HashSet::new());
    }

    output(config, contributors, contributor_files, i);
}

fn output(
    _config: ContributorsConfig,
    contributors: HashMap<ContributorKey, u32>,
    contributor_files: HashMap<ContributorKey, HashSet<String>>,
    _commit_count: i32,
) {
    let mut table = term_table::Table::new();
    table.max_column_width = 400;
    table.style = TableStyle::thin();
    table.add_row(term_table::row::Row::new(vec![
        term_table::table_cell::TableCell::new("Contributors"),
        term_table::table_cell::TableCell::new("Commits"),
        term_table::table_cell::TableCell::new("Files Touched"),
    ]));
    for k in contributors.keys() {
        let contributor = truncate_right(k.to_string(), 70);
        table.add_row(term_table::row::Row::new(vec![
            term_table::table_cell::TableCell::new(contributor),
            term_table::table_cell::TableCell::new_with_alignment(
                contributors[k],
                1,
                term_table::table_cell::Alignment::Right,
            ),
            term_table::table_cell::TableCell::new_with_alignment(
                contributor_files[k].len(),
                1,
                term_table::table_cell::Alignment::Right,
            ),
        ]));
    }
    println!("{}", table.render());
}
