use std::path::Path;

use rbook::Ebook;

use library_types::*;
use pdf_parser::*;
use epub_parser::*;
mod pdf_parser;
mod epub_parser;
type Cover = Option<Vec<u8>>;
pub fn parse_book(path: &Path, parent_uuid: &str) -> (ParseBook, Cover) {
  match path.extension().unwrap().to_str().unwrap() {
    //"pdf"  	=> 	parse_pdf(path, media),
    "epub" 	=> 	parse_epub(path, parent_uuid),
    _ 		=> 	panic!("Unknown file type in file parser!"), } }

pub fn get_uuid (path: &Path) -> Option<String> {
  match path.extension().unwrap().to_str().unwrap() {
    //"pdf"  	=> 	get_pdf_uuid(path),
    "epub" 	=> 	get_epub_uuid(path),
    _ 		=> 	panic!("Unknown file type in file parser!"), } }



