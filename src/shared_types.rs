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

impl Verbosity {
    pub fn is_quiet(&self) -> bool {
        match self {
            Verbosity::Silent => true,
            Verbosity::Error => true,
            _ => false
        }
    }

    pub fn is_not_quiet(&self) -> bool {
        !self.is_quiet()
    }

    pub fn is_verbose(&self) -> bool {
        match self {
            Verbosity::Debug => true,
            Verbosity::Trace => true,
            _ => false
        }
    }

    pub fn is_informative(&self) -> bool {
        match self {
            Verbosity::Info => true,
            Verbosity::Debug => true,
            Verbosity::Trace => true,
            _ => false
        }
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

pub struct SpecificMetrics {
    pub path: String,
    pub cyclomatic: Option<i64>,
    pub cognitive: Option<i64>,
    pub loc: Option<i64>
}