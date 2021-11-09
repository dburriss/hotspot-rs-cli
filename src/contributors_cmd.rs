use crate::shared_types::{ContributorsConfig};
use std::path::{Path};
use std::collections::{HashMap, HashSet};
use git2::{Repository};
// maybe this? https://docs.rs/git2/0.13.22/git2/struct.Repository.html#method.revwalk
// get files see code here: https://github.com/rust-lang/git2-rs/issues/588#issuecomment-856757971
// C# impl https://github.com/libgit2/libgit2sharp/pull/963/files

pub fn execute(config: ContributorsConfig) {
    let base_dir = Path::new(&config.repository_path);
    let repo = match Repository::open(base_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let mut contributors : HashMap<String,u32> = HashMap::new();
    let mut contributor_files : HashMap<String,HashSet<String>> = HashMap::new();
    let mut rev_walk = repo.revwalk().unwrap();
    //rev_walk.simplify_first_parent().unwrap();
    rev_walk.push_head().unwrap();
    let mut i = 0;
    for elem in rev_walk {
        let oid = elem.unwrap();
        let commit = repo.find_commit(oid).unwrap();
        
        let author = commit.author();
        let email = author.email().unwrap();
        let h = oid.to_string();
        //println!("{} {}", h, commit.summary().unwrap());
        // could use tree to get files?
        if commit.parent_count() == 0 {
            let tree = commit.tree().unwrap();
            let prev_tree = None;
            let diff = repo.diff_tree_to_tree(prev_tree, Some(&tree), None).unwrap();
            for delta in diff.deltas() {
                let file_path = delta.new_file().path().unwrap();
                let file_mod_time = commit.time();
                let unix_time = file_mod_time.seconds();
                //println!("{} modified at {}", file_path.to_str().unwrap(), unix_time);
                let h = contributor_files.entry(email.to_string()).or_insert(HashSet::new());
                h.insert(file_path.to_str().unwrap().to_string());
            }
        }
        if commit.parent_count() == 1 {
            let prev_commit = commit.parent(0).unwrap();
            let tree = commit.tree().unwrap();
            let prev_tree = prev_commit.tree().unwrap();
            let diff = repo.diff_tree_to_tree(Some(&prev_tree), Some(&tree), None).unwrap();
            for delta in diff.deltas() {
                let file_path = delta.new_file().path().unwrap();
                let file_mod_time = commit.time();
                let unix_time = file_mod_time.seconds();
                //println!("{} modified at {}", file_path.to_str().unwrap(), unix_time);
                let h = contributor_files.entry(email.to_string()).or_insert(HashSet::new());
                h.insert(file_path.to_str().unwrap().to_string());
            }
        }
        i = i + 1;
        *contributors.entry(email.to_string()).or_insert(0) += 1;
        let _ = *contributor_files.entry(email.to_string()).or_insert(HashSet::new());
    }

    for k in contributors.keys() {
        println!(
            "| Contributor: {:<60} |  Commits:{:<5} | Files: {:<4} |", k, contributors[k], contributor_files[k].len()
        );
    }

    println!("Total commits: {}", i);
}

pub fn _execute(config: ContributorsConfig) {
    let base_dir = Path::new(&config.repository_path);
    // https://docs.rs/git2/0.13.22/git2/struct.Repository.html#method.reflog
    let repo = match Repository::open(base_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let mut contributors : HashMap<String,u32> = HashMap::new();
    // let count = repo.references().unwrap().count();
    // println!("References: {}", count);
    let head =  repo.head().unwrap();
    let logs = repo.reflog(head.name().unwrap()); // https://docs.rs/git2/0.13.22/git2/struct.Reflog.html 

    logs.iter().for_each(|rlog| rlog.iter().for_each( |log_entry| { 
        let committer = log_entry.committer();
        let email = committer.email().unwrap().to_string();
        *contributors.entry(email).or_insert(0) += 1;
    }));

    for k in contributors.keys() {
        println!(
            "{} with {} commits", k, contributors[k]
        );
    }
}