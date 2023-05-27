mod pdf_parser;
use library_types::*;
use rbook::Ebook;
use std::path::Path;
use pdf_parser::*;

pub fn parse_media_file(path: &Path, parent_uuid: &str) -> (MediaFile, Option<Vec<u8>>) {
    let media = parse_media(path);
    (MediaFile::new(path, parent_uuid, media.0), media.1)
}


fn parse_media(path: &Path) -> (Media, Option<Vec<u8>>) {
    if path.is_dir() { return (Media::DirType, None); }

    match path.extension().unwrap().to_str().unwrap() {
        "pdf" => parse_pdf(path),
        "epub" => parse_epub(path),
        _ => panic!("Unknown file type in file parser!"),
    }
}

fn parse_epub(path: &Path) -> (Media, Option<Vec<u8>>) {
    let epub = rbook::Epub::new(path).unwrap();
    let metadata = epub.metadata();
    let epub = Epub::new(metadata.title().unwrap().value(),
        metadata.unique_identifier().unwrap().value(), );
    (Media::EpubType(epub), None)
}

