pub mod client_file_io;

use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use iter_tools::{Either, Itertools};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FILETYPES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("mp3", "audio");
        m
    };
}

pub struct ScannedDir {
    pub path: PathBuf,
    pub dirs: Vec<ScannedDir>,
    pub files: Vec<PathBuf>,
}

impl ScannedDir {
    pub fn new(path: PathBuf, dirs: Vec<ScannedDir>, files: Vec<PathBuf>) -> Self {
        ScannedDir { path, dirs, files, }
    }

    pub fn scan_path(path: &str) -> Self {
        ScannedDir::scan(PathBuf::from(path))
    }

    pub fn scan(path: PathBuf) -> Self {
        let paths = fs::read_dir(&path).unwrap()
            .map(|res| res.unwrap().path());

        let (dirs, files): (Vec<ScannedDir>, Vec<PathBuf>) =
            paths.partition_map(|path| {
                if path.is_dir() { Either::Left(ScannedDir::scan(path)) }
                else { Either::Right(path) }
            });

        ScannedDir::new(path, dirs, files)
    }

}