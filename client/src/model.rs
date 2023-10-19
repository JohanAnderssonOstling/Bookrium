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

pub fn get_covers(path: &str) -> Vec<String> {
    //Get all folders in path
    println!("{path}/.bookrium");;
    let library_path = PathBuf::from(format!("{path}/.bookrium"));
    let mut covers: Vec<String> = vec!["".to_string(); 4];
    let mut cover_count = 0;
    if let Ok(entries) = fs::read_dir(library_path) {
        for entry in entries {
            if let Ok(dir) = entry {
                if (dir.path().is_dir()) {
                    if let Some(cover) = get_cover(dir.path()) {
                        println!("Pushing {cover}");
                        covers[cover_count] = cover;
                        cover_count += 1;

                        if cover_count == 4 { break; }
                    }
                }
            }

        }
    }
    println!("Returning covers");
    covers
}

pub fn get_cover(mut path: PathBuf) -> Option<String> {
    path.push("128.jpg");
    //println!("{}",path.to_str().unwrap());
    if !path.exists() {return None;}
    Some(path.to_str().unwrap().to_string())
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


