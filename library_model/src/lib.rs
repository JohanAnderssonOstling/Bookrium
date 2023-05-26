use async_recursion::async_recursion;
use db_wrapper::surrealdb::library_db::{LibraryDBConn, Record};
use file_io::client_file_io::DATA_DIR;
use file_io::ScannedDir;
use file_parser::parse_media_file;
use library_types::*;
use std::fs;
use std::path::Path;
use tokio;

/// A representation of a library
pub struct LibraryModel {
    /// A connection to the database
    db: LibraryDBConn,
}

impl LibraryModel {
    /// Opens a database connection
    /// # Parameters
    /// 'uuid' - uuid of library in database 
    pub fn open(uuid: &str) -> Self {
        Self{
            db: LibraryDBConn::new(uuid).unwrap()
        }


    }
/*
    async fn open_mem(uuid: &str) -> LibraryModel {
        let db = LibraryDBConn::new_mem("mem://").await.unwrap();
        Self { db }
    }
*/
    /// Adds file to library; if file is not supported an error is returned.
    /// # Parameters
    /// `file` - metadata for file to add to library
    /// # Note:
    /// Upon scanning the same folder multiple times, the uuid will get updated due calling parse_media_file using the constructor
    pub fn add_file(&self, file: &MediaFile) -> Record {
        let insert_result = self.db.insert_media_file(file);
        let record = match insert_result {
            Ok(record) => record,
            Err(e) => panic!("Error inserting file into database: {}", e),
        };
        record
    }

    /// Gets all files in a library
    /// # Returns
    /// a vector containing the metadata of all files in the library
    pub fn get_files(&self) -> Vec<MediaFile> {
        println!("Getting files from library model");
        self.db.select_media_files().unwrap()
    }

    /// Recursively scans a directory and adds all the files to the library
    /// # Parameters
    /// `path` - the path to the directory to scan
    /// # Note:
    /// Upon scanning the same folder multiple times, the uuid will get updated due calling parse_media_file using the constructor
    pub fn add_files(&self, path: &str, library_uuid: &str) {

        fn add_files_aux(library: &LibraryModel, scanned_dir: ScannedDir, parent_uuid: &str, library_uuid: &str) {

            let mut files = Vec::new();

            for file in scanned_dir.files {
                println!("Adding file: {}", file.to_str().unwrap());
                let file = parse_media_file(&file, parent_uuid);
                library.add_file(&file);
                files.push(file);
            }

            for dir in scanned_dir.dirs {
                let dir_file = parse_media_file(&dir.path, parent_uuid);
                library.add_file(&dir_file);
                add_files_aux(library, dir, &dir_file.uuid, library_uuid);

            }

            let uuid = library_uuid.to_string();
            std::thread::spawn(move || {
                for file in files {
                    println!("Creating thumbnail for file:");
                    match file.media {
                        library_types::Media::PdfType(p) => {
                            let img = file_io::client_file_io::convert_img(p.cover);
                            if img.is_none() {
                                continue;
                            }
                            file_io::client_file_io::creating_thumbnails(
                                &uuid,
                                file.uuid.as_str(),
                                img.unwrap(),
                            )
                        }
                        _ => {} // TODO: add thumbnail for epub
                    }
                }
            });
        }

        let scanned_dir = ScannedDir::scan_path(path);
        add_files_aux(self, scanned_dir, "root", library_uuid);

    }

    /// # NOT YET IMPLEMENTED
    /// Removes a file from the library
    /// ## Parameters
    /// `uuid` - The uuid of the file to remove
    pub async fn remove_file(&self, uuid: &str) {
        self.db.delete_media_file(uuid).unwrap();
    }

    /// # NOT YET IMPLEMENTED
    /// Removes a file from the library
    pub fn edit_metadata(&self) {
        todo!()
    }
}
/*
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_add_files() {
        // NOT WORKING ?
        let db = LibraryModel::open("test", "ws://127.0.0.1:8000").await;
        let file = MediaFile::default();
        let original_files = db.get_files().await;
        let file_count = original_files.len();
        db.add_file(&file).await;
        let files = db.get_files().await;
        assert_eq!(files.len(), file_count + 1);
    }

    use super::*;
    #[tokio::test]
    async fn add_file() {
        let library_model = LibraryModel::open_mem("test").await;

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("sample_files")
            .join("sample1.epub");

        dbg!(&path);

        let file = file_parser::parse_media_file(path.as_path(), "root");
        library_model.add_file(&file).await;

        assert_eq!(library_model.get_files().await.len(), 1);
    }

    #[tokio::test]
    async fn add_files() {
        let library_model = LibraryModel::open_mem("test").await;

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sample_files");

        dbg!(&path);

        library_model
            .add_files(path.as_os_str().to_str().unwrap(), "test")
            .await;

        assert_eq!(library_model.get_files().await.len(), 6);
    }
}
*/