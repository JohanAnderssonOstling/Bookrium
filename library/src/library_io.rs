use std::collections::HashSet;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use image::io::Reader;
use iter_tools::{Either, Itertools};
use lazy_static::lazy_static;

lazy_static!(
  static ref FILETYPES: HashSet<&'static str> = HashSet::from(["epub", "pdf"]);
);

const THUMBNAIL_SIZES: [f32; 4] = [128.0, 256.0, 512.0, 1024.0];
const HEIGHT_RATIO: f32 = 1.6;


pub fn is_filetype(path: &PathBuf) -> bool {
    if	path.is_dir() { return true; }
    let extension = path.extension().unwrap().to_str().unwrap();
    FILETYPES.contains(extension)
}

pub fn scan_dir(path: &PathBuf) -> (Vec<PathBuf>, Vec<PathBuf>) {
    fs::read_dir(path).unwrap()
		      .map(|res| res.unwrap().path())
		      .filter(is_filetype)
		      .partition_map(|path| {
			  if path.is_dir() { Either::Left(path) } else { Either::Right(path) }
		      })
}


pub fn create_thumbnails(path: String, image_data: Vec<u8>){

    let image = Reader::new(Cursor::new(&image_data)).with_guessed_format().unwrap().decode().unwrap();
    for thumbnail_size in THUMBNAIL_SIZES.iter(){
	let thumbnail_height = thumbnail_size * HEIGHT_RATIO.clone();
	let thumbnail = image.thumbnail(thumbnail_size.clone() as u32, thumbnail_height as u32);
	thumbnail.save(format!("{path}/{thumbnail_size}.jpg")).unwrap();
    }
}