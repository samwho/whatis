mod infer;
mod noop;

use crate::whatis::FileType;
use std::{collections::HashSet, path::PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type MatcherFn = fn(&PathBuf) -> Result<Option<FileType>>;

static ALL: &[MatcherFn] = &[infer::matcher, noop::matcher];

pub fn match_all(path: &PathBuf) -> Result<HashSet<FileType>> {
    let mut possible_types: HashSet<FileType> = HashSet::new();
    for matcher in ALL {
        match matcher(&path) {
            Ok(Some(file_type)) => possible_types.insert(file_type),
            Ok(None) => continue,
            Err(e) => return Err(e.into()),
        };
    }

    Ok(possible_types)
}
