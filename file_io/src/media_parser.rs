use std::path::Path;

use rbook::Ebook;

use library_types::*;
use pdf_parser::*;

mod pdf_parser;

pub fn parse_media(path: &Path, parent_uuid: &str) -> (MediaFile, Option<Vec<u8>>) {
	let media = MediaFile::new(path, parent_uuid);
	if path.is_dir() { return (media, None); }

	match path.extension().unwrap().to_str().unwrap() {
		"pdf" => parse_pdf(path, media),
		"epub" => parse_epub(path, media),
		_ => panic!("Unknown file type in file parser!"),
	}

}

fn parse_epub(path: &Path, media: MediaFile) -> (MediaFile, Option<Vec<u8>>) {
	let epub = rbook::Epub::new(path).unwrap();
	let metadata = epub.metadata();

	let title = metadata.title().unwrap().value().to_string();
	let isbn = metadata.unique_identifier().unwrap().value().to_string();


	let cover_href = epub.cover_image().unwrap().value();
	let cover = epub.read_bytes_file(cover_href).ok();

	(media, cover)
}

