use crate::shared_types::{ContributorsConfig};
use std::path::{Path};
use std::collections::HashMap;
use git2::Repository;
// maybe this? https://docs.rs/git2/0.13.22/git2/struct.Repository.html#method.revwalk


pub fn execute(config: ContributorsConfig) {
    let base_dir = Path::new(&config.repository_path);
    let repo = match Repository::open(base_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let mut contributors : HashMap<String,u32> = HashMap::new();
    let mut rev_walk = repo.revwalk().unwrap();
    //rev_walk.simplify_first_parent().unwrap();
    rev_walk.push_head().unwrap();
    let mut i = 0;
    for elem in rev_walk {
        let oid = elem.unwrap();
        let commit = repo.find_commit(oid).unwrap();
        
        // could use tree to get files?
        let author = commit.author();
        let email = author.email().unwrap().to_string();
        let h = oid.to_string();
        println!("{} {}", h, commit.summary().unwrap());
        let tree = commit.tree().unwrap();
        for e in tree.iter() {
            println!("tree entry {} is {} ({})", e.name().unwrap(), e.kind().unwrap(), e.id())
        }
        i = i + 1;
        *contributors.entry(email).or_insert(0) += 1;
    }

    for k in contributors.keys() {
        println!(
            "{} with {} commits", k, contributors[k]
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