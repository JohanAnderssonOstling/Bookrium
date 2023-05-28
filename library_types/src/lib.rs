use std::path::*;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub enum MediaType {
    #[default]
    DirType,
    EbookType(Ebook),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct Epub {}


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum BookFormat {
    EpubType(Epub), PdfType(Pdf),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Ebook {
    pub title: String,
    pub isbn: String,
    pub book_format: BookFormat,
}

impl Ebook {
    pub fn new(title: String, isbn: String, book_format: BookFormat) -> Self {
        Self { title, isbn, book_format }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct Pdf {}


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct MediaFile {
    pub path: String,
    pub uuid: String,
    pub parent_dir_uuid: String,
    pub duration: String,
    pub position: String,
    pub media_type: MediaType,
}


impl MediaFile {
    pub fn new(path: &Path, parent_dir_uuid: &str) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            path: path.to_str().unwrap().to_string(),
            parent_dir_uuid: parent_dir_uuid.to_string(),
            ..Default::default()
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