use lazy_static::lazy_static;
use crate::epub::*;
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! (
    static ref EPUBS: Mutex<HashMap<String, Epub>> = Mutex::new(HashMap::new());
);

#[cxx::bridge]
mod ffi {

    extern "Rust" {
	fn open_epub(path: &str) -> String;
	fn next_chapter(uuid: &str);
        fn prev_chapter(uuid: &str);
        fn go_to(uuid: &str, href: &str);

	fn get_text(uuid: &str) -> String;
	fn add_paragraph(uuid: &str) -> String;
        fn add_prev_paragraph(uuid: &str) -> String;

	fn remove_paragraph(uuid: &str);
        fn remove_prev_paragraph(uuid: &str);
        fn reset_paragraph(uuid: &str);
        fn next_paragraphs(uuid: &str);
        fn prev_paragraphs(uuid: &str);
    }

}

fn open_epub (path: &str) -> String {
    let epub = Epub::new(path);
    let uuid = uuid::Uuid::new_v4().to_string();
    EPUBS.lock().unwrap().insert(uuid.clone(), epub);
    uuid
}

fn next_chapter(uuid: &str) {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.next_chapter();
}

fn prev_chapter(uuid: &str) {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.prev_chapter();
}

fn go_to (uuid: &str, href: &str) {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.go_to(href);
}

fn get_text (uuid: &str) -> String {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.get_text()
}

fn add_paragraph (uuid: &str) -> String {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.add_paragraph()
}

fn add_prev_paragraph (uuid: &str) -> String {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.add_prev_paragraph()
}

fn remove_paragraph (uuid: &str) {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.remove_paragraph();
}

fn remove_prev_paragraph (uuid: &str) {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.remove_prev_paragraph();
}

fn reset_paragraph (uuid: &str) {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.reset_paragraph();
}

fn next_paragraphs (uuid: &str) {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.next_paragraphs();
}

fn prev_paragraphs (uuid: &str) {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.prev_paragraphs();
}