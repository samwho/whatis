#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FileType {
    AAC,
    AbiWord,
    Arc,
    AVI,
    JPEG,
    PNG,
    PlainText,

    Unknown,
}

impl From<&str> for FileType {
    fn from(s: &str) -> Self {
        match s {
            "aac" | "audio/aac" => FileType::AAC,
            "abw" | "application/x-abiword" => FileType::AbiWord,
            "arc" | "application/x-freearc" => FileType::Arc,
            "avi" | "video/x-msvideo" => FileType::AVI,
            "jpeg" | "jpg" | "image/jpeg" => FileType::JPEG,
            "png" | "image/png" => FileType::PNG,

            _ => FileType::Unknown,
        }
    }
}
