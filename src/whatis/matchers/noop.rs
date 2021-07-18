use crate::whatis::{FileType, Result};
use std::path::PathBuf;

pub fn matcher(path: &PathBuf) -> Result<Option<FileType>> {
    Ok(None)
}
