use std::path::Path;
use pdf::any::AnySync;
use pdf::enc::StreamFilter;
use pdf::file::{FileOptions, ObjectCache, StreamCache, File};
use pdf::{object::*, PdfError};
use library_types::Pdf;


pub fn parse_pdf(path: &Path) -> Pdf {
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
