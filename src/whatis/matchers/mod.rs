mod example;
mod infer;

use crate::whatis::FileType;
use std::{collections::HashSet, path::Path};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type MatcherFn = fn(&Path) -> Result<Option<FileType>>;

// If you're writing a new matcher, you want to add your matcher function to
// this array.
static ALL: &[MatcherFn] = &[infer::matcher, example::matcher];

pub fn match_all(path: &Path) -> Result<HashSet<FileType>> {
    let mut matches: HashSet<FileType> = HashSet::new();
    for matcher in ALL {
        match matcher(&path) {
            Ok(Some(file_type)) => matches.insert(file_type),
            Ok(None) => continue,
            Err(e) => return Err(e),
        };
    }

    Ok(matches)
}
