use chrono::{DateTime, Utc};
#[allow(dead_code)]
pub struct RepositoryInfo {
    base: String,
    created_at: DateTime<Utc>,
    last_updated: DateTime<Utc>,
    is_git: bool
}
#[allow(dead_code)]
pub fn init(path: String) -> RepositoryInfo {
    unimplemented!("Nothing implemented to initialize for {}", path)
}