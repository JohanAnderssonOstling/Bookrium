use library_model::LibraryModel;
use library_types::{MediaFile};
use std::{
    collections::HashMap,
};
use std::ops::{Deref, DerefMut};


type UUID = String;
pub struct LibraryApp {
    // uuid as key
    open_libraries: HashMap<UUID, LibraryModel>,
}



impl LibraryApp {
    /// Creates an instance of this struct with no open libraries
    pub fn new() -> Self {
        Self {
            open_libraries: HashMap::new(),
        }
    }

    /// Opens a library
    /// # Parameters
    /// 'uuid' - the uuid of the library
    pub fn open(&mut self, uuid: &str, url: &str) {
        self.open_libraries.insert(uuid.to_string(), LibraryModel::open(uuid));
    }

    /// Adds a file to a library
    /// # Parameters
    /// 'uuid' - the uuid of the library
    /// 'file' - the file to add to the library
    /// # Errors
    /// Returns an error if a library with the uuid `uuid` wasn't found among the open libraries
    pub fn add_file(&mut self, uuid: &str, file: &MediaFile) -> Result<(), String> {
        let library_model = self.open_libraries.get(uuid)
            .ok_or("Library wasn't found")?;
        library_model.add_file(file);
        Ok(())
    }

    /// Adds all files (including those in subdirectories) to a library
    /// # Parameters
    /// 'uuid' - the uuid of the lilbrary
    pub fn add_files(&mut self, uuid: &str, path: &str) -> Result<(), String> {
        let library_model = self.open_libraries.get(uuid)
            .ok_or("Library wasn't found")?;
        library_model.add_files(path, uuid);

        Ok(())
    }

    /// Gets all the files in a library
    /// # Parameters
    /// 'uuid' - the uuid of the library
    /// # Errors
    /// Returns an error if a library with the uuid `uuid` wasn't found among the open libraries
    pub fn get_files(&mut self, uuid: &str) -> Result<Vec<MediaFile>, String> {
        let library_model = self.open_libraries.get(uuid)
            .ok_or("Library wasn't found")?;
        println!("Getting files from library app");
        Ok(library_model.get_files())
    }
}
/*
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use file_parser::parse_media_file;
    use library_types::Media;
    use crate::LibraryApp;

    #[tokio::test]
    async fn test_integration() {
        //Get path
        let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Get the path of the crate root
        test_dir.push("..");
        test_dir.push("test_directory");
        let mut path_dir = test_dir.clone();
        path_dir.push("pg70413.epub");
        let path = path_dir.as_path();

        //Parse file
        let media_file = parse_media_file(path, "parent uuid");
        assert_ne!(media_file.uuid.len(), 0);
        let media = &media_file.media;
        let title = &match media {
            Media::EpubType(epub) => {
                epub.title.clone()
            }
            _ => panic!("Not epub media!")
        };
        assert_eq!(title, "The presidental snapshot");

        //Add file to library
        let mut library_app = LibraryApp::new();
        library_app.open("library", "ws://127.0.0.1:8000").await;
        let original_files_result = library_app.get_files("library").await;
        let original_files = match original_files_result {
            Ok(files) => files,
            Err(e) => panic!("Failed to files: {}", e)
        };
        let file_count = original_files.len();
        library_app.add_file("library", &media_file).await.expect("Failed to add file");

        //Get file from library
        let new_files_result = library_app.get_files("library").await;
        let new_files = match new_files_result {
            Ok(files) => files,
            Err(e) => panic!("Failed to files: {}", e),
        };

        //Compare data
        assert_eq!(new_files.len(), file_count + 1);
        let new_file = new_files.last().unwrap();
        assert_eq!(new_file.media, *media);
    }
}
*/