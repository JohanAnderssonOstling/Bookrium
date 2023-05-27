use std::fs;
use std::collections::HashSet;
use std::path::PathBuf;
use iter_tools::{Either, Itertools};
use lazy_static::lazy_static;


lazy_static!(
  static ref FILETYPES: HashSet<&'static str> = HashSet::from(["epub", "pdf"]);
);

pub struct ScannedDir {
    pub dirs: Vec<PathBuf>, pub files: Vec<PathBuf>,
}

impl ScannedDir {
    pub fn new(dirs: Vec<PathBuf>, files: Vec<PathBuf>) -> Self {
        ScannedDir { dirs, files, }
    }
}

pub fn is_filetype(path: &PathBuf) -> bool {
    let extension = path.extension().unwrap().to_str().unwrap();
    FILETYPES.contains(extension) || path.is_dir()
}

pub fn scan_dir(path: PathBuf) -> ScannedDir {
    let (dirs, files): (Vec<PathBuf>, Vec<PathBuf>) = fs::read_dir(&path).unwrap()
        .map(|res| res.unwrap().path())
        .filter(|path| is_filetype(path))
        .partition_map(|path| {
            if path.is_dir() { Either::Left(path) }
            else { Either::Right(path) }
        });
    ScannedDir::new(dirs, files)
}

