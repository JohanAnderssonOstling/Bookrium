use std::path::PathBuf;
use async_recursion::async_recursion;
use log::info;
use db_wrapper::surrealdb::library_db::{LibraryDBConn, Record};
use file_io::file_io::*;
use file_io::folder_scanner::{scan_dir};
use file_io::media_parser::parse_media_file;
use library_types::*;


pub struct LibraryModel {
    db: LibraryDBConn,
    pub uuid: String,
}

impl LibraryModel {
    pub fn open(uuid: &str) -> Self {
        Self{ db: LibraryDBConn::new(uuid).unwrap() , uuid: uuid.to_string()}
    }
/*
    async fn open_mem(uuid: &str) -> LibraryModel {
        let db = LibraryDBConn::new_mem("mem://").await.unwrap();
        Self { db }
    }
*/
    pub async fn add_file(&self, file: &MediaFile){
        let insert_result = self.db.insert_file(file).await;
        let record = match insert_result {
            Ok(record) => record,
            Err(e) => panic!("Error inserting file into database: {}", e),
        };
    }

    pub async fn fetch_files(&self) -> Vec<MediaFile> {
        self.db.select_files().await.unwrap()
    }

    pub async fn scan_library(&self, path: &str) {
        let files = self.db.clear_library().await.unwrap();
        let scan_path = PathBuf::from(path);
        self.scan_library_aux(scan_path, "root").await;
    }

    #[async_recursion]
    async fn scan_library_aux(&self, scan_path: PathBuf, parent_uuid: &str) {
        let scanned_dir = scan_dir(scan_path);

        for dir in scanned_dir.0 {
            let dir_type = Media::DirType;
            let dir_file = MediaFile::new(dir.as_path(), parent_uuid, dir_type);
            self.add_file(&dir_file).await;
            self.scan_library_aux(dir, &dir_file.uuid).await;
        }

        for file in scanned_dir.1 {
            let parsed_file = parse_media_file(&file, parent_uuid);
            let file = parsed_file.0;
            self.add_file(&file).await;
            if parsed_file.1 != None {
                create_thumbnails_raw(&self.uuid, &file.uuid, parsed_file.1.unwrap()).await;
            }
        }
    }

}
