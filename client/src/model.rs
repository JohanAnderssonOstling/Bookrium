use std::fs::OpenOptions;
use library_types::home_types::Library;
use csv::*;

const CSV_PATH: &str = "/home/johan/.local/share/media_library/home.csv";


pub fn create_library(path: &str) -> Library {
    let library = Library::new(path);
    let file = OpenOptions::new().create(true).append(true)
                                 .open(CSV_PATH).unwrap();
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    writer.serialize(&library).unwrap();
    writer.flush().unwrap();
    library
}

pub fn get_libraries() -> Vec<Library> {
    let mut reader = Reader::from_path(CSV_PATH).unwrap();
    reader.deserialize().map(|result| result.unwrap()).collect()
}

pub fn delete_library(uuid: &str) {
    //self.conn.delete_library(uuid).await.unwrap();
    todo!()
}

pub fn get_library(uuid: &str) -> Library {
    //self.conn.select_library(uuid).await.unwrap()
    todo!()
}

