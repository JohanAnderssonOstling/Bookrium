use std::path::Path;
use rbook::Ebook;
use rbook::epub::Metadata;
use rbook::xml::Element;
use library_types::*;
use crate::media_parser::*;

type Elems<'a> = Vec<&'a Element>;

pub fn parse_epub(path: &Path, mut media: Book) -> (Book, Cover) {
  let epub = 			rbook::Epub::new(path).unwrap();
  let meta = 			epub.metadata();

  media.title = 		get_elem(meta.title());
  media.desc = 			get_elem(meta.description());
  media.contents=		parse_contents(epub.toc().elements());
  media.subjects=		parse_subjects(meta.subject());
  media.ids = 			parse_identifiers(meta);

  let cover_href = 		epub.cover_image().unwrap().value();
  let cover = 			epub.read_bytes_file(cover_href).ok();

  (media, cover) }

fn get_elem (elem: Option<&Element>) -> String {
  match elem {
    Some(elem) 	=> elem.value().to_string(),
    None 	=> String::new(), } }

fn parse_contents (elems: Elems) -> Contents{
  let mut nav = Vec::new();
  for item in elems {
    let childs = parse_contents (item.children());
    nav.push(Nav::new(item.name(), item.value(), childs)); }
  nav }

fn parse_subjects		(elems: Elems) -> Subjects {
  let mut subjects = 	Vec::new();
  for elem in elems {
    subjects.push(		Subject::new (elem.value())); };
  subjects }

fn parse_creators		(elems: Elems) -> Creators {
  let mut creators = 	Vec::new();
  for elem in elems {
    //creators.push(Creator::new(elem.value()));
  };
  creators }


fn parse_identifiers	(meta: &Metadata) -> IDs {
  let mut identifiers: 	Vec<Identifier> = 		Vec::new();
  for elem in meta.get	("identifier") {
    let val = 			elem.value().to_string();
    let identifier = 	match elem.name().to_lowercase() {
	  id if id.contains	("isbn") => Identifier::ISBN(val),
      id if id.contains	("asin") => Identifier::Asin(val),
      id if id.contains	("goog") => Identifier::GOOG(val),
      _ 					=> Identifier::None, };
    identifiers.push	(identifier); }
  identifiers
}
