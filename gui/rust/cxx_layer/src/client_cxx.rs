use ffi::Library;

fn start_db() {
    client_app::start_db();
}

fn create_library(name: &str, path: &str, url: &str) -> Library{
    convert_library(client_app::create_library(name, path, url))
}

fn get_libraries() -> Vec<Library>{
    let rust_libraries = client_app::get_libraries();
    let mut cxx_libraries = Vec::new();
    for rust_library in rust_libraries {
        cxx_libraries.push(convert_library(rust_library));
    }
    cxx_libraries
}

fn delete_library(uuid: &str){
    client_app::delete_library(uuid)
}

fn convert_library(library: library_types::Library) -> Library {
    Library { uuid: library.uuid, name: library.name, path: library.path}
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn create_library(name: &str, path: &str, url: &str) -> Library;
        fn get_libraries() -> Vec<Library>;
        fn delete_library(uuid: &str);
        fn start_db();
    }

    pub struct Library {pub uuid: String, pub name: String, pub path: String}
}