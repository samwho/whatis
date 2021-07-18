use crate::whatis::{FileType, Result};
use std::path::Path;

impl From<infer::Type> for FileType {
    fn from(it: infer::Type) -> Self {
        it.extension().into()
    }
}

pub fn matcher(path: &Path) -> Result<Option<FileType>> {
    match infer::get_from_path(path)? {
        Some(infer_type) => Ok(Some(infer_type.into())),
        None => Ok(None),
    }
}
