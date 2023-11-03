use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default, Clone)]
pub struct Library {
    pub uuid: String,
    pub path: String,
}

impl Library {
	pub fn new(path: &str) -> Self {
		Self {
				uuid: Uuid::new_v4().to_string(),
		    	path: path.to_string(),

		}
	}
}