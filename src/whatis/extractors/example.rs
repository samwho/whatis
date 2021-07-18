/// This file exists to serve as a template for how to write an extractor. Feel
/// free to copy it as a starting point for your own extractors.
use crate::whatis::Result;
use serde_json::{Map, Value};
use std::path::Path;

pub fn extractor(_: &Path) -> Result<Option<Map<String, Value>>> {
    Ok(None)
}
