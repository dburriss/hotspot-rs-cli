extern crate globwalk;

use self::globwalk::{DirEntry, GlobWalker, WalkError};

pub fn setup_file_walker(
    base_dir: &Path,
) -> FilterMap<GlobWalker, fn(Result<DirEntry, WalkError>) -> Option<DirEntry>> {
    globwalk::GlobWalkerBuilder::from_patterns(
        base_dir,
        &[
            "*.{cs,csx,c,cpp,fs,fsx,go,js,java,py,rs,ts,tsx}",
            "!.*",
            "!node_modules/",
            "!target/",
        ],
    )
    .build()
    .unwrap()
    .into_iter()
    .filter_map(Result::ok)
}
