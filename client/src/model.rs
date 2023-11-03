use std::fmt::format;
use std::fs;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use library_types::home_types::Library;
use csv::*;
use std::fs::*;

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

pub fn get_covers(path: &str, cover_scale: u32) -> Vec<String> {
    let library_path = PathBuf::from(format!("{path}/.bookrium"));
    let mut covers: Vec<String> = vec!["".to_string(); 2];
    let mut cover_count = 0;
    if let Ok(entries) = fs::read_dir(library_path) {
        for entry in entries {
            if let Ok(dir) = entry {
                if dir.path().is_dir() {
                    let cover_name = format!("{}.jpg", 128 * cover_scale);
                    let mut cover_path = dir.path();
                    cover_path.push(cover_name);
                    if !cover_path.exists() {continue}
                    covers[cover_count] = cover_path.to_str().unwrap().to_string();
                    cover_count += 1;
                    if cover_count == 2 { break; }
                }
            }
        }
    }
    covers
}

pub fn modify_library(modified_library: Library) {
    let libraries = get_libraries();
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(OpenOptions::new().create(true).truncate(true)
                                       .write(true).open(CSV_PATH).unwrap());
    writer.serialize(("uuid", "path")).unwrap();
    for library in libraries {
        if library.uuid != modified_library.uuid{
            writer.serialize(&library).unwrap();
        }
        else {
            writer.serialize(&modified_library).unwrap();
        }
    }
    writer.flush().unwrap();
}

pub fn get_cover(mut path: PathBuf) -> Option<String> {
    path.push("256.jpg");
    if !path.exists() {return None;}
    Some(path.to_str().unwrap().to_string())
}

pub fn delete_library(uuid: &str) {
    //Delete csv entry matching uuid
    let mut reader = Reader::from_path(CSV_PATH).unwrap();
    let libraries: Vec<Library> = reader.deserialize().map(|result| result.unwrap()).collect();

    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(OpenOptions::new().create(true).truncate(true)
                                       .write(true).open(CSV_PATH).unwrap());
    writer.serialize(("uuid", "path")).unwrap();
    for library in libraries {
        if library.uuid != uuid{
            writer.serialize(&library).unwrap();

        }
    }
    writer.flush().unwrap();
}


