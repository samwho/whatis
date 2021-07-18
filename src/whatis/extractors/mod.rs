mod generic_image;
mod jpeg;
mod png;

use std::path::Path;

use super::FileType;
use serde_json::{Map, Value};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type ExtractorFn = fn(&Path) -> Result<Option<Map<String, Value>>>;

pub trait Extractor {
    fn extract(&self, path: &Path) -> Result<Option<Map<String, Value>>>;
}

impl Extractor for FileType {
    fn extract(&self, path: &Path) -> Result<Option<Map<String, Value>>> {
        let result = match self {
            FileType::JPEG => combine(&[generic_image::extractor, jpeg::extractor], path)?,
            FileType::PNG => combine(&[generic_image::extractor, png::extractor], path)?,
            _ => return Ok(None),
        };

        Ok(result)
    }
}

fn combine(fns: &[ExtractorFn], path: &Path) -> Result<Option<Map<String, Value>>> {
    let mut ret: Option<Map<String, Value>> = None;
    for f in fns {
        if let Some(mut data) = f(path)? {
            if ret.is_none() {
                ret = Some(data);
            } else if let Some(m) = ret.as_mut() {
                m.append(&mut data);
            }
        }
    }

    Ok(ret)
}
