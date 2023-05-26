use library_types::*;
use pdf::any::AnySync;
use pdf::enc::StreamFilter;
use pdf::file::{FileOptions, ObjectCache, StreamCache, File};
use pdf::{object::*, PdfError};
use rbook::Ebook;
use std::path::Path;
use std::sync::Arc;

/// Calls methods for getting general metadata and specific metadata, returns File struct
/// # Parameters
/// `path` - The path to the file
/// # Returns
/// MediaFile - A struct containing the metadata of the file
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

/// Parses a file and returns a Media struct
/// # Parameters
/// `path` - The path to the file
/// # Returns
/// Media - A struct containing the metadata of the file
fn parse_media(path: &Path) -> Media {
    if path.is_dir() {
        return Media::DirType;
    }

    match path.extension().unwrap().to_str().unwrap() {
        "epub" => Media::EpubType(parse_epub(path)),
        "pdf" => Media::PdfType(parse_pdf(path)),
        _ => panic!("Unknown file type in file parser!"),
    }
}

/// Parses an epub file and returns an Epub struct
/// # Parameters
/// `path` - The path to the epub file
/// # Returns
/// Epub - A struct containing the metadata of the epub file
fn parse_epub(path: &Path) -> Epub {
    let epub = rbook::Epub::new(path).unwrap();
    let metadata = epub.metadata();
    Epub::new(
        metadata.title().unwrap().value(),
        metadata.unique_identifier().unwrap().value(),
    )
}

/// Parses a pdf file and returns a Pdf struct
/// # Parameters
/// `path` - The path to the pdf file
/// # Returns
/// Pdf - A struct containing the metadata of the pdf file
fn parse_pdf(path: &Path) -> Pdf {
    let file = FileOptions::cached().open(path).unwrap();
    let info = file.trailer.info_dict.as_ref().unwrap();

    let title = info.get("Title").and_then(|p| p.to_string_lossy().ok()).unwrap_or("".to_string());
    let author = info
        .get("Author")
        .and_then(|p| p.to_string_lossy().ok())
        .unwrap_or("".to_string());
    let page_count = info
        .get("Pages")
        .and_then(|p| p.as_integer().ok())
        .unwrap_or(0) as u32;
    let isbn = info
        .get("ISBN")
        .or_else(|| info.get("isbn"))
        .and_then(|p| p.to_string_lossy().ok())
        .unwrap_or("".to_string());
    let creator = info
        .get("Creator")
        .or_else(|| info.get("creator").or_else(|| info.get("CREATOR")))
        .and_then(|p| p.to_string_lossy().ok())
        .unwrap_or("".to_string());

    let cover = get_cover(&file).expect("Error when trying to get the cover the pdf file");

    Pdf::new("".to_string(), 0, title, author, isbn, page_count, creator, cover)
}


/// Gets the cover of a pdf file
/// # Parameters
/// `file` - The file to get the cover from
/// # Returns
/// Vec<u8> - The cover of the pdf file
pub fn get_cover(file: &File<Vec<u8>, ObjectCache, StreamCache>) -> Result<Vec<u8>, PdfError> {
    let first_page = file.get_page(0)?;
    let resources = first_page.resources()?;
    let mut images: Vec<_> = vec![];

    images.extend(resources.xobjects.iter()
        .map(|(_name, &r)| file.get(r).unwrap())
        .filter(|o| matches!(**o, XObject::Image(_)))
    );

    for o in images.iter() {
        let img = match **o {
            XObject::Image(ref im) => im,
            _ => continue
        };
        let (data, filter) = img.raw_image_data(file)?;
        let ext = match filter {
            Some(StreamFilter::DCTDecode(_)) => "jpeg",
            Some(StreamFilter::JBIG2Decode) => "jbig2",
            Some(StreamFilter::JPXDecode) => "jp2k",
            _ => continue,
        };

        return Ok(Vec::from(data.as_ref()));
    }
    Ok(Vec::new())

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_pdf() {
        let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Get the path of the crate root
        test_dir.push("..");
        test_dir.push("test_directory");
        let mut path_dir = test_dir.clone();
        path_dir.push("X59.pdf");
        let pdf = parse_pdf(path_dir.as_path());

        assert_eq!(pdf.title, "X59");
        assert_eq!(pdf.author, "Simon Pislar");
        assert_eq!(pdf.creator, "LaTeX");
        assert_eq!(pdf.page_count, 0);
        assert_eq!(pdf.isbn, "");
        assert_eq!(pdf.cover.len(), 161615);
    }
}

/*
#[cfg(test)]
mod tests {
    use std::path::{PathBuf};
    use std::time::{UNIX_EPOCH};
    use rbook::Ebook;
    use library_types::*;
    use crate::{get_epub_metadata, get_file_type, get_metadata};

    //Test get if title is found correctly in epub file
    #[test]
    fn test_get_metadata_from_epub() {
        let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Get the path of the crate root
        test_dir.push("..");
        test_dir.push("test_directory");
        let mut path_dir = test_dir.clone();
        path_dir.push("pg70413.epub");
        let path = path_dir.as_path();
        let metadata: EpubMetadata = get_epub_metadata(path); //fix path
        let epub = rbook::Epub::new(path).unwrap();
        let title = match epub.metadata().title() {
            Some(title) => title.value().to_string(),
            None => panic!("Could not find title for epub!")
        };
        let identifier = match epub.metadata().unique_identifier() {
            Some(identifier) => identifier.value().to_string(),
            None => panic!("Could not find identifier for epub!")
        };
        assert_eq!(title, metadata.title);
        assert_eq!(identifier, metadata.isbn);

        let metadata: EpubMetadata = get_epub_metadata(path_dir.as_path());

        assert_eq!(metadata.title, "The presidental snapshot");
        assert_eq!(metadata.isbn, "http://www.gutenberg.org/70413");
    }

    //Test get general information from file
    #[test]
    fn test_get_metadata_from_file() {
        let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Get the path of the crate root
        test_dir.push("..");
        test_dir.push("test_directory");
        let mut path_dir = test_dir.clone();
        path_dir.push("pg70413.epub");
        let path = path_dir.as_path();

        let metadata: MediaFile = get_metadata(&path);
        let path_metadata = path.metadata().unwrap_or_else(|_| panic!("Cannot get metadata of {:?}", path));
        let creation_date = path_metadata.created().unwrap_or_else(|_| panic!("Cannot get creation time of {:?}", path));
        let modified_date = path_metadata.modified().unwrap_or_else(|_| panic!("Cannot get modified time of {:?}", path));


        let filetype: FileType = get_file_type(test_dir.join("pg70413.epub").as_path());
        assert_eq!(creation_date.duration_since(UNIX_EPOCH)
                       .expect("Time went backwards")
                       .as_millis() as u32, metadata.creation_date);

        assert_eq!(modified_date.duration_since(UNIX_EPOCH)
                       .expect("Time went backwards")
                       .as_millis() as u32, metadata.modified_date);
        let metadata: MediaFile = get_metadata(&path);

        //assert_eq!(1367039273, metadata.creation_date);
        //assert_eq!(1367039277, metadata.modified_date);
        assert_eq!(metadata.file_type, FileType::Epub);
    }
}
*/
