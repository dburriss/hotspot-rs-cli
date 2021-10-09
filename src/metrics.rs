use crate::shared_types::MetricsConfig;

pub fn execute(config: MetricsConfig) {
    println!("Path: {}", config.repository_path);
    println!("Verbosity: {}", config.verbosity);

}