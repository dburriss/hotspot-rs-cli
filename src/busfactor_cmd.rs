use crate::shared_types::{truncate, BusFactorConfig, ContributorKey};
use git2::Repository;
use std::collections::{HashMap, HashSet};

use std::path::Path;

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

    let mut contributors: HashMap<ContributorKey, u32> = HashMap::new();
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
                // if file changes : https://docs.rs/git2/latest/git2/struct.DiffDelta.html

                // StatusDelta (Delta) : https://docs.rs/git2/latest/git2/enum.Delta.html
            }
        }
    }
}
