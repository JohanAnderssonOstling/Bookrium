pub mod home_types;
mod epub_type;
use chrono::{NaiveDate};
use std::path::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


pub struct Dir {
	pub uuid: String,
	pub name: String,
	pub path: String,
	pub parent_uuid: String,
}

#[derive(Default)]
pub struct Book {
	pub uuid: String,
	pub path: String,
	pub desc: String,
	pub title: String,
	pub duration: String,
	pub position: String,
	pub published: NaiveDate,
	pub parent_dir_uuid: String,

	pub navigation: Vec<Nav>,
	pub identifiers: Vec<Identifier>,

	pub creators: Vec<Creator>,
	pub subjects: Vec<Subject>,

}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub enum Identifier {
	#[default] NoIdentifier,
	ISBN (String),
	Asin (String),
	GOOG (String),
}


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct Subject {
	pub uuid: String,
	pub name: String,
}
impl Subject {
	pub fn new(name: &str) -> Self {
		Self { uuid: uuid(), name: name.to_string(), }
	}
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct Creator {
	pub uuid: String,
	pub name: String,
	pub role: CreatorRole,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub enum CreatorRole {
	#[default] Author,
	Translator,
	Contributor,
	Narrator,
}

pub struct Publisher {
	pub uuid: String,
	pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct Nav {
	pub name: String,
	pub href: String,
	pub childs: Vec<Nav>,
}
impl Nav {
	pub fn new(name: &str, href: &str, childs: Vec<Nav>) -> Self {
		Self {name: name.to_string(), href: href.to_string(), childs}
	}
}

impl Book {
	pub fn new(path: &Path, parent_dir_uuid: &str) -> Self {
		Self {
			uuid: uuid(),
			path: path.to_str().unwrap().to_string(),
			parent_dir_uuid: parent_dir_uuid.to_string(),
			..Default::default()
		}
	}
}

fn uuid() -> String {
	Uuid::new_v4().to_string()
}


