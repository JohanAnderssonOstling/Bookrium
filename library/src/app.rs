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
    models: HashMap<UUID, LibraryModel>,
}

impl LibraryApp {
    /// Creates an instance of this struct with no open libraries
    pub fn new() -> Self {
        Self { models: HashMap::new() }
    }

    pub fn open(&mut self, uuid: &str, url: &str) {
        self.models.insert(uuid.to_string(), LibraryModel::open(uuid));
    }

    pub fn add_file(&mut self, uuid: &str, file: &MediaFile){
        let library_model = self.models.get(uuid).unwrap();
        RUNTIME.block_on(library_model.add_file(file));
    }

    pub fn add_files(&mut self, uuid: &str, path: &str){
        let library_model = self.models.get(uuid).unwrap();
        RUNTIME.block_on(library_model.scan_library(path));
    }


    pub fn get_files(&mut self, uuid: &str) -> Vec<MediaFile> {
        let library_model = self.models.get(uuid).unwrap();
        RUNTIME.block_on(library_model.fetch_files())
    }
}
