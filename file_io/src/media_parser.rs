use std::path::Path;

use rbook::Ebook;

use library_types::*;
use pdf_parser::*;
use epub_parser::*;
mod pdf_parser;
mod epub_parser;

pub fn parse_media(path: &Path, parent_uuid: &str) -> (MediaFile, Option<Vec<u8>>) {
	let media = MediaFile::new(path, parent_uuid);
	if path.is_dir() { return (media, None); }

	match path.extension().unwrap().to_str().unwrap() {
		"pdf" => parse_pdf(path, media),
		"epub" => parse_epub(path, media),
		_ => panic!("Unknown file type in file parser!"),
	}

}



