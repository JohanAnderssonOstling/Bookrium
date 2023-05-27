use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use iter_tools::{Either, Itertools};
use lazy_static::lazy_static;

lazy_static!(
  static ref FILETYPES: HashSet<&'static str> =
		HashSet::from(["epub", "pdf"]);
);

pub fn is_filetype(path: &PathBuf) -> bool {
	let extension = path.extension().unwrap().to_str().unwrap();
	path.is_dir() || FILETYPES.contains(extension)
}

pub fn scan_dir(path: PathBuf) -> (Vec<PathBuf>, Vec<PathBuf>) {
	fs::read_dir(&path).unwrap()
			.map(|res| res.unwrap().path())
			.filter(|path| is_filetype(path))
			.partition_map(|path| {
				if path.is_dir() { Either::Left(path) } else { Either::Right(path) }
			})
}

