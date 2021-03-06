mod extractors;
mod file_type;
mod matchers;

pub use file_type::FileType;
use serde_json::{Map, Value};
use std::path::Path;

use self::extractors::Extractor;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn whatis(path: &Path) -> Result<Option<(FileType, Map<String, Value>)>> {
    for file_type in matchers::match_all(path)? {
        match file_type.extract(path)? {
            Some(data) => return Ok(Some((file_type, data))),
            None => continue,
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("testdata/Canon_40D.jpg" => FileType::JPEG)]
    #[test_case("testdata/rustlogo.png" => FileType::PNG)]
    fn test_match(path: &str) -> FileType {
        let (file_type, _) = whatis(&Path::new(path)).unwrap().unwrap();
        file_type
    }
}
