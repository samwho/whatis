/// This file exists to serve as a template for how to write a matcher. Feel
/// free to copy it as a starting point for your own matchers.
use crate::whatis::{FileType, Result};
use std::path::Path;

pub fn matcher(_: &Path) -> Result<Option<FileType>> {
    Ok(None)
}
