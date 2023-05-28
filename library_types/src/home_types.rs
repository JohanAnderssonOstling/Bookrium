use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default, Clone)]
pub struct Library {
	pub uuid: String,
	pub name: String,
	pub path: String,
	pub url: String
}

impl Library {
	pub fn new(name: &str, path: &str, url: &str) -> Self {
		Self { uuid: Uuid::new_v4().to_string(), name: name.to_string(),
			path: path.to_string(), url: url.to_string(),

		}
	}
}