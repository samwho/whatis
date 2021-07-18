use self::FileType::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FileType {
    JPEG,
    PNG,
    Unknown,
}

impl From<&str> for FileType {
    fn from(s: &str) -> Self {
        match s {
            "jpeg" | "jpg" | "image/jpeg" => JPEG,
            "png" | "image/png" => PNG,
            rest => {
                log::debug!(
                    "attempted to convert unknown string in to FileType: {}",
                    rest
                );
                Unknown
            }
        }
    }
}
