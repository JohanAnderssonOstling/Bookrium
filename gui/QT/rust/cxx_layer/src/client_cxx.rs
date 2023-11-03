use ffi::Library;
use client::*;

fn create_library(path: &str) -> Library{
    convert_library(model::create_library(path))
}

fn get_libraries() -> Vec<Library>{
    let rust_libraries = model::get_libraries();
    let mut cxx_libraries = Vec::new();
    for rust_library in rust_libraries {
        cxx_libraries.push(convert_library(rust_library));
    }
    cxx_libraries
}

fn get_covers(library_path: &str, cover_scale: u32) -> Vec<String> {
    model::get_covers(library_path, cover_scale)
}

fn delete_library(uuid: &str){
    model::delete_library(uuid)
}

fn convert_library(lib: library_types::home_types::Library) -> Library {
    Library {
        uuid: lib.uuid,
        name: lib.path.split("/").last().unwrap().to_string(),
        path: lib.path
    }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn create_library(path: &str) -> Library;
        fn get_libraries() -> Vec<Library>;
        fn get_covers(library_path: &str, cover_scale: u32) -> Vec<String>;
        fn delete_library(uuid: &str);
    }

    pub struct Library {pub uuid: String, pub name: String, pub path: String}
}