use core::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Verbosity {
    Silent,
    Error,
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
pub struct MetricsConfig {
    pub repository_path: String,
    pub verbosity: Verbosity,
    pub output: String,
    pub includes: String,
    pub excludes: String
}
