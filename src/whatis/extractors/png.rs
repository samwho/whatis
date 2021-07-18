use crate::whatis::Result;
use exif::parse_exif;
use img_parts::png::Png;
use img_parts::ImageEXIF;
use serde_json::{Map, Value};
use std::path::Path;

pub fn extractor(path: &Path) -> Result<Option<Map<String, Value>>> {
    let mut data = Map::new();
    let mut exif = Map::new();
    let png = Png::from_bytes(std::fs::read(path)?.into())?;

    if let Some(buf) = png.exif() {
        let (fields, _) = parse_exif(&buf)?;

        for field in fields {
            exif.insert(
                field.tag.to_string(),
                Value::String(field.value.display_as(field.tag).to_string()),
            );
        }

        data.insert("exif".to_string(), Value::Object(exif));
        return Ok(Some(data));
    }

    Ok(None)
}
