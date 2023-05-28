use std::path::Path;
use rbook::Ebook;
use rbook::xml::Element;
use library_types::{MediaFile, Navigation, NavPoint};

pub fn parse_epub(path: &Path, mut media: MediaFile) -> (MediaFile, Option<Vec<u8>>) {
	let epub = rbook::Epub::new(path).unwrap();
	let metadata = epub.metadata();

	let isbn = metadata.unique_identifier().unwrap().value().to_string();

	media.title = metadata.title().unwrap().value().to_string();
	media.description = metadata.description().unwrap().value().to_string();
	media.navigation = parse_navigation(&epub);

	let cover_href = epub.cover_image().unwrap().value();
	let cover = epub.read_bytes_file(cover_href).ok();

	(media, cover)
}

fn parse_navigation(epub: &rbook::Epub) -> Navigation{
	let toc: Vec<&Element> = epub.toc().elements();
	Navigation{ nav_points: parse_nav_points(toc)}
}

fn parse_nav_points(elem: Vec<&Element>) -> Vec<NavPoint> {
	let mut nav_points: Vec<NavPoint> = Vec::new();
	for item in elem {
		nav_points.push(NavPoint {
			name: item.name().to_string(),
			href: item.value().to_string(),
			children: parse_nav_points(item.children()),
		});

	}
	nav_points
}

//unit tests

#[cfg(test)]
mod tests {
	use std::env;
	use std::path::PathBuf;
	use rbook::Epub;
	use super::*;

	fn get_epub_path() -> PathBuf {
		let mut epub_path: PathBuf = env::current_dir().unwrap();
		epub_path.push("src");
		epub_path.push("media_parser");
		epub_path.push("test_files");
		epub_path.push("Philosophy of Software Design.epub");
		epub_path
	}

	fn setup()  -> Epub {
		rbook::Epub::new(get_epub_path().as_path()).unwrap()
	}




}