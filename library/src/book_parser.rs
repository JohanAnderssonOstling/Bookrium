use std::path::Path;

use library_types::*;
use pdf_parser::*;
use epub_parser::*;
mod pdf_parser;
mod epub_parser;

type Cover = Option<Vec<u8>>;

pub fn parse_book (path: &Path) -> Option<(ParseBook, Cover)> {
  let pat = path.to_str().unwrap();
  match path.extension().unwrap().to_str().unwrap() {
    "pdf" 	=> 	parse_pdf(path),
    "epub" 	=> 	parse_epub(path),
    _ 		=> 	panic!("Unknown file type in file parser!"),
  }
}

pub fn get_uuid (path: &Path) -> Option<String> {
  let path_str = path.to_str().unwrap();
  match path.extension().unwrap().to_str().unwrap() {
    //"pdf"  	=> 	get_pdf_uuid(path),
    "epub" => get_epub_uuid(path),
    _ => None
  }
}