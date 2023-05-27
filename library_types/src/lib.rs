use std::path::*;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub enum Media {
    #[default]
    DirType,
    EpubType(Epub), PdfType(Pdf),
    Mp3Type(Mp3),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct Epub {
    pub title: String,
    pub isbn: String,
}

impl Epub {
    pub fn new(title: &str, isbn: &str) -> Epub {
        Epub { title: title.to_string(), isbn: isbn.to_string(), }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct Pdf {
    pub current_page: u32,      pub page_count: u32,
    pub title: String,
    pub author: String,         
    pub isbn: String,

}

impl Pdf {
    pub fn new(title: String, author: String, isbn: String, page_count: u32) -> Self {
        Self { current_page: 0, page_count, title, author, isbn }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct Mp3 {
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct MediaFile {
    pub path: String, pub uuid: String,
    pub parent_dir_uuid: String,
    pub media: Media,
    // TODO: add access_date in future
}

fn to_unix_time(system_time: SystemTime) -> u32 {
    system_time.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        as u32
}


impl MediaFile {
    pub fn new(path: &Path, parent_dir_uuid: &str, media: Media) -> Self {

        Self {
            path: path.to_str().unwrap().to_string(),
            uuid: Uuid::new_v4().to_string(),
            parent_dir_uuid: parent_dir_uuid.to_string(),
            media,
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default, Clone)]
pub struct Library {
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub url: String
}

impl Library {
    pub fn new(name: &str, path: &str, url: &str) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            name: name.to_string(),
            path: path.to_string(),
            url: url.to_string(),
        }
    }
}