use chrono::{DateTime, Utc};

pub struct RepositoryInfo {
    base: String,
    created_at: DateTime<Utc>,
    last_updated: DateTime<Utc>,
    is_git: bool
}

pub fn init(path: String) -> RepositoryInfo {
    unimplemented!("Nothing implemented to initialize for {}", path)
}