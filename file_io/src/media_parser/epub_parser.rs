use std::path::Path;
use pdf::content::Op;
use rbook::Ebook;
use rbook::epub::Metadata;
use rbook::xml::Element;
use library_types::*;
use crate::media_parser::*;

type Elems<'a> = Vec<&'a Element>;

pub fn parse_epub(path: &Path, mut media: Book) -> (Book, Cover) {
	let epub = rbook::Epub::new(path).unwrap();
	let metadata: &Metadata = epub.metadata();

	media.title = get_elem(metadata.title());
	media.desc = get_elem(metadata.description());
	media.navigation = parse_nav(epub.toc().elements());
	media.subjects = parse_subjects(metadata.subject());
	media.identifiers = parse_identifiers(metadata);

	let cover_href = epub.cover_image().unwrap().value();
	let cover = epub.read_bytes_file(cover_href).ok();

	(media, cover)
}

fn get_elem(elem: Option<&Element>) -> String {
	elem.unwrap().value().to_string()
}

fn parse_nav(elems: Elems) -> Vec<Nav>{
	let mut nav: Vec<Nav> = Vec::new();
	for item in elems {
		let childs = parse_nav(item.children());
		nav.push(Nav::new(item.name(), item.value(), childs));
	}
	nav
}


fn parse_subjects(elems: Elems) -> Vec<Subject> {
	let mut subjects: Vec<Subject> = Vec::new();
	for elem in elems {
		subjects.push(Subject::new(elem.value()));
	};
	subjects
}

fn parse_creators(elems: Elems) -> Vec<Creator> {
	let mut creators: Vec<Creator> = Vec::new();
	for elem in elems {
		//creators.push(Creator::new(elem.value()));
	};
	creators
}


fn parse_identifiers(meta: &Metadata) -> Vec<Identifier> {
	let mut identifiers: Vec<Identifier> = Vec::new();
	for elem in meta.get("identifier") {
		let val = elem.value().to_string();
		let identifier = match elem.name().to_lowercase() {
			id if id.contains("isbn") => Identifier::ISBN(val),
			id if id.contains("asin") => Identifier::Asin(val),
			id if id.contains("goog") => Identifier::GOOG(val),
			_ => Identifier::NoIdentifier,
		};
		identifiers.push(identifier);
	}
	identifiers
}
