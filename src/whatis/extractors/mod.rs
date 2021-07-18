mod generic_image;
mod jpeg;
mod png;

use std::path::PathBuf;

use super::FileType;
use serde_json::{Map, Value};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type ExtractorFn = fn(&PathBuf) -> Result<Option<Map<String, Value>>>;

pub trait Extractor {
    fn extract(&self, path: &PathBuf) -> Result<Option<Map<String, Value>>>;
}

impl Extractor for FileType {
    fn extract(&self, path: &PathBuf) -> Result<Option<Map<String, Value>>> {
        let result = match self {
            FileType::JPEG => combine(&[generic_image::extractor, jpeg::extractor], path)?,
            FileType::PNG => combine(&[generic_image::extractor, png::extractor], path)?,
            _ => return Ok(None),
        };

        Ok(result)
    }
}

fn combine(fns: &[ExtractorFn], path: &PathBuf) -> Result<Option<Map<String, Value>>> {
    let mut ret: Option<Map<String, Value>> = None;
    for f in fns {
        if let Some(mut data) = f(path)? {
            if ret.is_none() {
                ret = Some(data);
            } else {
                ret.as_mut().map(|m| m.append(&mut data));
            }
        }
    }

    Ok(ret)
}
