use lazy_static::lazy_static;
use crate::epub::*;
use std::sync::{Mutex, MutexGuard};
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
        fn get_pos(uuid: &str) -> String;
	fn set_pos(uuid: &str, pos: &str);
    }
}

fn get_epub<F: FnOnce(&mut Epub)>(uuid: &str, f: F) {
    let mut epubs = EPUBS.lock().unwrap();
    if let Some(epub) = epubs.get_mut(uuid) {
        f(epub);
    }
}

fn open_epub (path: &str) -> String {
    let path = path.to_string().replace("\\\"", "\\");
    let epub = Epub::new(path.as_str());
    let uuid = uuid::Uuid::new_v4().to_string();
    EPUBS.lock().unwrap().insert(uuid.clone(), epub);
    uuid
}

fn next_chapter(uuid: &str)     { get_epub(uuid, |epub| epub.next_chapter()); }
fn prev_chapter(uuid: &str)     { get_epub(uuid, |epub| epub.prev_chapter()); }
fn go_to(uuid: &str, href: &str){ get_epub(uuid, |epub| epub.go_to(href)); }

fn get_text(uuid: &str) -> String {
    let mut text = String::new();
    get_epub(uuid, |epub| text = epub.get_text());
    text
}

fn add_paragraph(uuid: &str) -> String {
    let mut text = String::new();
    get_epub(uuid, |epub| text = epub.add_paragraph());
    text
}

fn add_prev_paragraph(uuid: &str) -> String {
    let mut text = String::new();
    get_epub(uuid, |epub| text = epub.add_prev_paragraph());
    text
}

fn remove_paragraph(uuid: &str)     { get_epub(uuid, |epub| epub.remove_paragraph()); }
fn remove_prev_paragraph(uuid: &str){ get_epub(uuid, |epub| epub.remove_prev_paragraph()); }
fn reset_paragraph(uuid: &str)      { get_epub(uuid, |epub| epub.reset_paragraph()); }

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

fn get_pos (uuid: &str) -> String {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.get_pos()
}

fn set_pos (uuid: &str, pos: &str) {
    let mut epubs = EPUBS.lock().unwrap();
    let epub = epubs.get_mut(uuid).unwrap();
    epub.set_pos(pos);
}