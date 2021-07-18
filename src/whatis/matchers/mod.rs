mod infer;
mod noop;

use crate::whatis::FileType;
use std::{collections::HashSet, path::Path};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type MatcherFn = fn(&Path) -> Result<Option<FileType>>;

static ALL: &[MatcherFn] = &[infer::matcher, noop::matcher];

pub fn match_all(path: &Path) -> Result<HashSet<FileType>> {
    let mut possible_types: HashSet<FileType> = HashSet::new();
    for matcher in ALL {
        match matcher(&path) {
            Ok(Some(file_type)) => possible_types.insert(file_type),
            Ok(None) => continue,
            Err(e) => return Err(e),
        };
    }

    Ok(possible_types)
}
