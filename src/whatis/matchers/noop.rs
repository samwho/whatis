use crate::whatis::{FileType, Result};
use std::path::Path;

pub fn matcher(_: &Path) -> Result<Option<FileType>> {
    Ok(None)
}
