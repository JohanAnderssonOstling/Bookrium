use std::path::Path;
use pdf::file::{FileOptions, ObjectCache, StreamCache, File};
use pdf::{object::*, PdfError};
use pdf::primitive::Dictionary;
use library_types::*;


pub fn parse_pdf(path: &Path, mut media: Book) -> (Book, Option<Vec<u8>>) {
    let file = FileOptions::cached().open(path).unwrap();
    let info = file.trailer.info_dict.as_ref().unwrap();

    let title = get_str_property(info, "Title");
    let isbn = get_str_property(info, "ISBN");
    media.len = get_str_property(info, "Pages");

    let cover = get_cover(&file).ok();
    (media, cover)
}

fn get_str_property(info: &Dictionary, key: &str) -> String {
    info.get(key).and_then(|p| p.to_string_lossy().ok()).
        unwrap_or("".to_string())
}

fn get_cover(file: &File<Vec<u8>, ObjectCache, StreamCache>) -> Result<Vec<u8>, PdfError> {
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
        return Ok(Vec::from(data.as_ref()));
    }
    Ok(Vec::new())
}

