use std::path::*;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;



pub struct Dir {
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub parent_uuid: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct MediaFile {
    pub uuid: String,
    pub path: String,
    pub duration: String,
    pub position: String,
    pub parent_dir_uuid: String,
}


impl MediaFile {
    pub fn new(path: &Path, parent_dir_uuid: &str) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            path:   path.to_str().unwrap().to_string(),
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