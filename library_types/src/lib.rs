pub mod home_types;
mod epub_type;

use std::path::*;
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
    pub uuid: String,       pub path: String,
    pub duration: String,   pub position: String,
    pub navigation: Navigation,
    pub parent_dir_uuid: String,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct Navigation {
    pub nav_points: Vec<NavPoint>,
}

pub struct Identifier {
    pub id: String,
    pub scheme: String,
}

pub enum Scheme {
    ISBN,
    MobiAsin,
    GOOGLE,
}

pub struct Subject {
    pub uuid: String,
    pub name: String,
}

pub struct Creator {
    pub uuid: String,
    pub name: String,
    pub creator_type: CreatorType,
}

pub enum CreatorType {
    Author,
}

pub struct Publisher {
    pub uuid: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct NavPoint {
    pub name: String,
    pub href: String,
    pub children: Vec<NavPoint>,
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


