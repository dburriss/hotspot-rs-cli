use core::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Verbosity {
    Silent,
    Info,
    Debug,
    Trace
}

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
// TODO: A context object that is told about each step starting, ending, progress, etc.
struct BaseConfig {
    operation: String,
    repository_path: String,
    verbosity: Verbosity,
    output: String,
    includes: String,
    excludes: String
}
