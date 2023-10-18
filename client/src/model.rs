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
    //Delete csv entry matching uuid
    let mut reader = Reader::from_path(CSV_PATH).unwrap();
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(OpenOptions::new().create(true).truncate(true)
                                      .write(true).open(CSV_PATH).unwrap());
    writer.serialize(("uuid", "path")).unwrap();
    for result in reader.records() {
        let record = result.unwrap().as_slice().to_string();
        if record != uuid {
            writer.serialize(record).unwrap();
        }
    }
    writer.flush().unwrap();
}


