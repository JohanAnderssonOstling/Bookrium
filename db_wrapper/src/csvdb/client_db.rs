use std::fs::File;
use library_types::Library;
use csv::*;
pub struct ClientDBConn {
    pub path: String
}

impl ClientDBConn {
    pub fn open(path: &str) -> Self {

        Self {
            path: path.to_string()
        }
    }
    pub fn insert_library(&self, library: &Library) {
        let mut writer = csv::Writer::from_path(&self.path).expect("Error opening csv file");
        writer.serialize(library).expect("Error writing library to csv");
    }
    pub fn select_libraries(&self) -> Vec<Library>{
        let mut reader = csv::Reader::from_path(&self.path).expect("Error opening csv file");
        reader.deserialize().map( |result| result.unwrap()).collect()
    }

}