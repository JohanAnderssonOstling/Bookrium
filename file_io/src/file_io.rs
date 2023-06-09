use directories::ProjectDirs;
use lazy_static::lazy_static;
use std::fs;
use std::io::Result as IoResult;
use std::path;
use std::path::PathBuf;
use std::io::*;
use image::{DynamicImage, io::Reader as ImageReader};use std::io::Cursor;

lazy_static! {
    pub static ref PROJECT_DIRS: ProjectDirs = ProjectDirs::from("com", "josh", "media_library").unwrap();
    pub static ref DATA_DIR: String = PROJECT_DIRS.data_dir().to_str().unwrap().to_string();
    pub static ref LIBRARY_DIR: String = format!("{}/libraries", DATA_DIR.as_str());
    pub static ref DB_PATH: String = format!("{}/client.db", DATA_DIR.as_str());
}

pub fn create_client_files() {
    fs::create_dir_all(DATA_DIR.clone()).unwrap()
}

pub fn create_library_files(library_path_str: &str) -> IoResult<()> {
    let library_path = path::Path::new(library_path_str);
    if !library_path.is_dir() {
        return Err(Error::new(ErrorKind::NotFound, "Directory not found"));
    }

    let library_data_path = format!("{}/{}",
        DATA_DIR.as_str(),
        library_path.file_name().unwrap().to_str().unwrap()
    );
    fs::create_dir_all(library_data_path)?;

    Ok(())
}

pub fn convert_img(image_data: Vec<u8>) -> Option<DynamicImage>{
    let reader = ImageReader::new(Cursor::new(image_data)).with_guessed_format().expect("Cursor io never fails");
    reader.decode().ok()
}

pub fn create_thumbs(library_uuid: &str, file_uuid: &str, image_data: Vec<u8>){
    let image = convert_img(image_data).unwrap();
    create_thumbnails(library_uuid, file_uuid, image);
}

pub fn create_thumbnails(library_uuid: &str, file_uuid: &str, image: DynamicImage){
    let thumbnail_sizes: Vec<f32> = vec![128.0,256.0,512.0,1024.0];

    let height_ratio: f32 = 1.6;
    let path = format!("{}/{}/{}/thumbnails", LIBRARY_DIR.as_str(), library_uuid, file_uuid);
    let path_buf = PathBuf::from(&path);

    fs::create_dir_all(&path).unwrap();
    for thumbnail_size in thumbnail_sizes.iter(){
        let thumbnail_height = thumbnail_size * height_ratio.clone();
        let thumbnail = image.thumbnail(thumbnail_size.clone() as u32, thumbnail_height as u32);
        thumbnail.save(&path_buf.join(format!("{}.jpg", thumbnail_size))).unwrap();
    }
}
