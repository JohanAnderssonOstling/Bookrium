mod pdf_parser;
use library_types::*;
use rbook::Ebook;
use std::path::Path;
use pdf_parser::*;

pub fn parse_media_file(path: &Path, parent_uuid: &str) -> MediaFile {
    let file_metadata = path.metadata().unwrap();
    MediaFile::new(
        path,
        file_metadata.created().unwrap(),
        file_metadata.modified().unwrap(),
        parent_uuid.to_string(),
        parse_media(path),
    )
}


fn parse_media(path: &Path) -> Media {
    if path.is_dir() { return Media::DirType; }

    match path.extension().unwrap().to_str().unwrap() {
        "epub" => Media::EpubType(parse_epub(path)),
        "pdf" => Media::PdfType(parse_pdf(path)),
        _ => panic!("Unknown file type in file parser!"),
    }
}

fn parse_epub(path: &Path) -> Epub {
    let epub = rbook::Epub::new(path).unwrap();
    let metadata = epub.metadata();
    Epub::new(metadata.title().unwrap().value(),
        metadata.unique_identifier().unwrap().value(), )
}

