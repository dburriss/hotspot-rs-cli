use crate::shared_types::{ContributorsConfig};
use std::path::{Path};
use std::collections::HashMap;
use git2::Repository;

pub fn execute(config: ContributorsConfig) {
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
        *contributors.entry(email).or_insert(1) += 1;
    }));

    for k in contributors.keys() {
        println!(
            "{} with {} commits", k, contributors[k]
        );
    }
}