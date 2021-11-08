use crate::shared_types::{ContributorsConfig};
use std::path::{Path};
use git2::Repository;

pub fn execute(config: ContributorsConfig) {
    let base_dir = Path::new(&config.repository_path);
    // https://docs.rs/git2/0.13.22/git2/struct.Repository.html#method.reflog
    let repo = match Repository::open(base_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let count = repo.references().unwrap().count();
    println!("References: {}", count);
    let head =  repo.head().unwrap();
    let log = repo.reflog(head.name().unwrap()); // https://docs.rs/git2/0.13.22/git2/struct.Reflog.html 

    let log_entry = log.iter().nth(0).unwrap().iter().nth(0).unwrap(); // https://docs.rs/git2/0.13.22/git2/struct.ReflogEntry.html
    let commit_message = log_entry.message().unwrap();
    let committer = log_entry.committer(); // https://docs.rs/git2/0.13.22/git2/struct.Signature.html
    println!(
        "Run contributors on {} at {} - message: {} from {}", 
        &config.repository_path, 
        head.name().unwrap(),
        commit_message,
        committer.name().unwrap()
    );
}