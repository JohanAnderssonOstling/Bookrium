use std::path::Path;
use rbook::{Ebook, Epub};
use rbook::epub::Metadata;
use rbook::xml::Element;
use library_types::*;
use crate::media_parser::*;

type Elems<'a> = Vec<&'a Element>;

pub fn get_epub_uuid(path: &Path) -> Option<String> {
    let epub_result = rbook::Epub::new(path);
    if epub_result.is_err() { return None; }
    let epub = epub_result.unwrap();
    let meta = epub.metadata();
    meta.unique_identifier().map(|id| id.value().into())
}

pub fn parse_epub(path: &Path, parent_dir: &str) -> (ParseBook, Cover) {
    println!("Parsing epub: {:?}", path);
    let epub = rbook::Epub::new(path).unwrap();
    let meta = epub.metadata();

    let book = ParseBook {
	book: parse_b(meta),
	mdata: parse_book_mdata(&epub, meta),
	name: path.file_name().unwrap().to_str().unwrap().into(),
	authors: get_elems(meta.creators()),
	subjects: get_elems(meta.subject()),
	publisher: get_elems(meta.publisher()),
    };

    let href = epub.cover_image();
    if href.is_none() { return (book, None); }
    let cover = epub.read_bytes_file(href.unwrap().value()).ok();
    (book, cover)
}

fn parse_b(mdata: &Metadata) -> LibBook {
    LibBook {
	uuid: get_elem(mdata.unique_identifier()),
	title: get_elem(mdata.title()),
	progress: 0,
    }
}

fn parse_book_mdata(epub: &Epub, mdata: &Metadata) -> BookMData {
    BookMData {
	desc: get_elem(mdata.description()),
	pos: "".into(),
	publ: get_date(mdata.date()),
	ids: parse_identifiers(mdata),
	contents: parse_contents(epub.toc().elements()),
    }
}

fn get_date(elem: Option<&Element>) -> u32 {
    match elem {
	Some(elem) => {
	    let mut val = elem.value();
	    if val.len() > 10 {
		val = elem.value().split_at(10).0;
	    }
	    val.replace("-", "").parse().unwrap()
	}
	None => 0,
    }
}

fn get_elem(elem: Option<&Element>) -> String {
    match elem {
	Some(elem) => elem.value().to_string(),
	None => String::new(),
    }
}

fn get_elems(elems: Elems) -> Vec<String> {
    let mut strings: Vec<String> = Vec::new();
    for elem in elems {
	strings.push(elem.value().to_string());
    }
    strings
}

fn parse_contents(elems: Elems) -> Contents {
    let mut nav = Vec::new();
    for item in elems {
	let childs = parse_contents(item.children());
	nav.push(Nav::new(item.name(), item.value(), childs));
    }
    nav
}


fn parse_identifiers(meta: &Metadata) -> IDs {
    let mut identifiers: IDs = Vec::new();
    for elem in meta.get("identifier") {
	let val = elem.value().to_string();
	let id_scheme = elem.attributes().first().unwrap().value();
	let identifier = match id_scheme.to_uppercase() {
	    id if id.contains("ISBN") => Identifier::ISBN(val),
	    id if id.contains("ASIN") => Identifier::Asin(val),
	    id if id.contains("GOOG") => Identifier::GOOG(val),
	    _ => Identifier::None,
	};
	if identifier != Identifier::None { identifiers.push(identifier); }
    }
    identifiers
}
