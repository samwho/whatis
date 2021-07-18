use crate::whatis::Result;
use image::GenericImageView;
use serde_json::{Map, Value};
use std::path::Path;

pub fn extractor(path: &Path) -> Result<Option<Map<String, Value>>> {
    let img = image::open(path)?;
    let mut data = Map::new();

    data.insert("width".to_string(), Value::String(img.width().to_string()));
    data.insert(
        "height".to_string(),
        Value::String(img.height().to_string()),
    );
    data.insert(
        "bits_per_pixel".to_string(),
        Value::String(img.color().bits_per_pixel().to_string()),
    );

    Ok(Some(data))
}
