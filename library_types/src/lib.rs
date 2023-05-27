use std::path::*;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub enum Media {
    #[default]
    DirType,
    EbookType(Ebook),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct Epub {

}

impl Epub {
    pub fn new() -> Self {
        Self {}
    }
}

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
pub struct Pdf {
    pub current_page: u32, pub page_count: u32,
}

impl Pdf {
    pub fn new(page_count: u32) -> Self {
        Self { current_page: 0, page_count }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct MediaFile {
    pub path: String,
    pub uuid: String,
    pub parent_dir_uuid: String,
    pub media: Media,
}


impl MediaFile {
    pub fn new(path: &Path, parent_dir_uuid: &str, media: Media) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            path: path.to_str().unwrap().to_string(),
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