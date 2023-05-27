use library_types::{MediaFile};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use lazy_static::lazy_static;
use crate::model::LibraryModel;
use tokio::runtime::Runtime;
lazy_static!(
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    );

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


    pub fn open(&mut self, uuid: &str, url: &str) {
        self.open_libraries.insert(uuid.to_string(), LibraryModel::open(uuid));
    }


    pub fn add_file(&mut self, uuid: &str, file: &MediaFile) -> Result<(), String> {
        let library_model = self.open_libraries.get(uuid)
            .ok_or("Library wasn't found")?;
        RUNTIME.block_on(library_model.add_file(file));
        Ok(())
    }

    pub fn add_files(&mut self, uuid: &str, path: &str) -> Result<(), String> {
        let library_model = self.open_libraries.get(uuid)
            .ok_or("Library wasn't found")?;
        RUNTIME.block_on(library_model.scan_library(path, uuid));

        Ok(())
    }


    pub fn get_files(&mut self, uuid: &str) -> Result<Vec<MediaFile>, String> {
        let library_model = self.open_libraries.get(uuid)
            .ok_or("Library wasn't found")?;
        println!("Getting files from library app");
        Ok(RUNTIME.block_on(library_model.fetch_files()))
    }
}
