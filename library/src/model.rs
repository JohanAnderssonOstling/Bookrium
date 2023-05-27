use std::path::PathBuf;
use async_recursion::async_recursion;
use db_wrapper::surrealdb::library_db::{LibraryDBConn, Record};
use file_io::file_io::*;
use file_io::folder_scanner::{scan_dir};
use file_io::media_parser::parse_media_file;
use library_types::*;
use library_types::Media::PdfType;


pub struct LibraryModel {
    db: LibraryDBConn,
}

impl LibraryModel {
    pub fn open(uuid: &str) -> Self {
        Self{ db: LibraryDBConn::new(uuid).unwrap() }
    }
/*
    async fn open_mem(uuid: &str) -> LibraryModel {
        let db = LibraryDBConn::new_mem("mem://").await.unwrap();
        Self { db }
    }
*/
    pub async fn add_file(&self, file: &MediaFile) -> Record {
        let insert_result = self.db.insert_media_file(file).await;
        let record = match insert_result {
            Ok(record) => record,
            Err(e) => panic!("Error inserting file into database: {}", e),
        };
        record
    }

    pub async fn fetch_files(&self) -> Vec<MediaFile> {
        self.db.select_media_files().await.unwrap()
    }

    pub async fn scan_library(&self, path: &str, library_uuid: &str) {
        let scan_path = PathBuf::from(path);
        self.scan_library_aux(scan_path, "root", library_uuid).await;
    }
    #[async_recursion]
    async fn scan_library_aux(&self, scan_path: PathBuf, parent_uuid: &str, library_uuid: &str) {
        let scanned_dir = scan_dir(scan_path);

        for dir in scanned_dir.dirs {
            let dir_file = parse_media_file(&dir, parent_uuid);
            self.add_file(&dir_file).await;
            self.scan_library_aux(dir, &dir_file.uuid, library_uuid).await;
        }
        for file in scanned_dir.files {
            let file = parse_media_file(&file, parent_uuid);
            self.add_file(&file).await;
            match file.media {
                PdfType(pdf) => {
                    create_thumbnails_raw(library_uuid, &file.uuid, pdf.cover).await;
                },
                _ => {},
            }
        }
    }

}
