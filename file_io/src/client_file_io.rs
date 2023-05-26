use directories::ProjectDirs;
use lazy_static::lazy_static;
use std::fs;
use std::io::Result as IoResult;
use std::path;
use std::path::PathBuf;
use image::{DynamicImage, io::Reader as ImageReader};use std::io::Cursor;

lazy_static! {
    pub static ref PROJECT_DIRS: ProjectDirs =
        ProjectDirs::from("com", "josh", "media_library").unwrap();

    pub static ref DATA_DIR: String = PROJECT_DIRS.data_dir().to_str().unwrap().to_string();
    pub static ref LIBRARY_DIR: String = format!("{}/libraries", DATA_DIR.as_str());
    pub static ref CLIENT_DB_PATH: String =
        format!("{}/client.db", PROJECT_DIRS.data_dir().to_str().unwrap());
}

pub fn create_client_files() {
    fs::create_dir_all(DATA_DIR.clone()).unwrap()
}

pub fn create_library_files(library_path_str: &str) -> IoResult<()> {
    let library_path = path::Path::new(library_path_str);
    if !library_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Directory not found",
        ));
    }

    let library_data_path = format!(
        "{}/{}",
        DATA_DIR.as_str(),
        library_path.file_name().unwrap().to_str().unwrap()
    );
    fs::create_dir_all(library_data_path)?;

    Ok(())
}

pub fn convert_img(image_data: Vec<u8>) -> Option<DynamicImage>{
    let reader = ImageReader::new(Cursor::new(image_data)).with_guessed_format().expect("Cursor io never fails");
    let image_result = reader.decode();
    let image = match image_result {
        Ok(image) => Some(image),
        Err(e) => None,
    };
    image
}

pub fn creating_thumbnails(library_uuid: &str, file_uuid: &str, image: DynamicImage){
    let thumbnail_sizes: Vec<u32> = vec![128,256,512,1024];

    let height_ratio:f32 = 1.6;
    let path = format!("{}/{}/{}/thumbnails", LIBRARY_DIR.as_str(), library_uuid, file_uuid);
    let path_buf = PathBuf::from(&path);

    fs::create_dir_all(&path).unwrap();
    println!("Creating thumbnails at: {}", path);
    for thumbnail_size in thumbnail_sizes{
        let thumbnail = image.thumbnail(thumbnail_size, (thumbnail_size.clone() as f32 * height_ratio.clone()) as u32);
        thumbnail.save(&path_buf.join(format!("{}.jpg", thumbnail_size))).unwrap();
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_thumbnails() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Get the path of the crate root
        test_path.push("test_images");
        test_path.push("Lebhar.jpg");
        let img = ImageReader::open(test_path).expect("error1").decode().expect("error2");
        let library_uuid = "1337";
        let file_uuid = "1337";
        let thumbnail_sizes = vec![128,256,512,1024];
        create_thumbnails(library_uuid, file_uuid, img);
        let path: String = format!("{}/{}/{}/thumbnails",LIBRARY_DIR.as_str(), library_uuid, file_uuid);
        let path_buf = PathBuf::from(&path);
        for thumbnail_size in thumbnail_sizes.iter(){
            assert!(&path_buf.join(format!("{}.jpg", thumbnail_size)).exists());
        }
    }
}*/