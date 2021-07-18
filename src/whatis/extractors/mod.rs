mod example;
mod generic_image;
mod jpeg;
mod png;

use std::path::Path;

use super::FileType::{self, *};
use serde_json::{Map, Value};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Extractor {
    fn extract(&self, path: &Path) -> Result<Option<Map<String, Value>>>;
}

impl Extractor for FileType {
    fn extract(&self, path: &Path) -> Result<Option<Map<String, Value>>> {
        // If you're writing a new extractor, you want to add your extractor to
        // this match. If no arm exists for what you want to match, add one.
        let result = match self {
            JPEG => combine(&[generic_image::extractor, jpeg::extractor], path)?,
            PNG => combine(&[generic_image::extractor, png::extractor], path)?,

            // This match arm does nothing, but demonstrates what a match arm
            // should look like.
            Unknown => example::extractor(path)?,
        };

        Ok(result)
    }
}

type ExtractorFn = fn(&Path) -> Result<Option<Map<String, Value>>>;
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
