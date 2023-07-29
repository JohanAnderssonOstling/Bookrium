

use rbook::{Ebook, Reader};

pub struct Epub {
    path: String,
    current_text: Vec<String>,
    chapter_index: usize,
    paragraph_index: usize,
    end_paragraph_index: usize,
}

impl Epub {
    pub fn new(path: &str) -> Self {
	Self {
	    path: path.to_string(),
	    current_text: Vec::new(),
	    chapter_index: 0,
	    paragraph_index: 0,
	    end_paragraph_index: 0,
	}
    }

    pub fn go_to (&mut self, href: &str) {
	let page = href.split("#").collect::<Vec<&str>>()[0];
	let id = href.split("#").collect::<Vec<&str>>()[1];
	let epub = rbook::Epub::new(&self.path).unwrap();
	let mut reader = epub.reader();
	println!("page: {}", page);

	reader.set_current_page_str(format!("text/{}", page).as_str()).unwrap().unwrap();
	self.chapter_index = reader.current_index();
	self.paragraph_index = 0;
	self.end_paragraph_index = 0;
	self.current_text = parse_paragraphs(reader);
	for (i, paragraph) in self.current_text.iter().enumerate() {
	    if paragraph.contains(format!("id=\"{}\"", id).as_str()) {
		self.paragraph_index = i;
		self.end_paragraph_index = i;
	    }
	}
    }


    pub fn next_chapter(&mut self) {

	let epub = rbook::Epub::new(&self.path).unwrap();
	let mut reader = epub.reader();
	if (reader.page_count() - 1) == self.chapter_index {
	    return
	}

	self.chapter_index 	+= 1;
	self.paragraph_index 	= 0;
	self.end_paragraph_index= 0;
	reader.set_current_page(self.chapter_index.clone()).unwrap().unwrap();
	self.current_text = parse_paragraphs(reader);
    }

    pub fn prev_chapter(&mut self) {
	if self.chapter_index == 0 { return }

	let epub = rbook::Epub::new(&self.path).unwrap();
	let mut reader = epub.reader();
	self.chapter_index -= 1;
	reader.set_current_page(self.chapter_index.clone()).unwrap().unwrap();
	self.current_text 	= parse_paragraphs(reader);
	self.paragraph_index 	= self.current_text.len() ;
	self.end_paragraph_index= self.current_text.len() ;
    }

    pub fn add_paragraph(&mut self) -> String {
	if self.end_paragraph_index >= self.current_text.len() {
	    return "EOF".into();
	}
	self.end_paragraph_index += 1;
	self.current_text[self.end_paragraph_index-1].clone()
    }

    pub fn add_prev_paragraph(&mut self) -> String {
	if self.paragraph_index == 0 {
	    return "BOF".into();
	}
	self.paragraph_index -= 1;
	self.current_text[self.paragraph_index].clone()
    }

    fn get_paragraph(&self, index: usize) -> String {
	let paragraph = self.current_text[index].clone();
	todo!()
    }

    pub fn remove_paragraph(&mut self) {
	if self.end_paragraph_index > self.paragraph_index {
	    self.end_paragraph_index -= 1;
    	}
    }

    pub fn remove_prev_paragraph(&mut self) {
	if self.paragraph_index < self.end_paragraph_index {
	    self.paragraph_index += 1;
	}
    }

    pub fn next_paragraphs(&mut self) {
	if self.end_paragraph_index + 1 >= self.current_text.len()  {
	    println!("End of chapter")	;
	    return self.next_chapter();
	}
	self.paragraph_index = self.end_paragraph_index;
    }

    pub fn prev_paragraphs(&mut self) {
	if self.paragraph_index == 0 {
	    return self.prev_chapter();
	}
    }

    pub fn reset_paragraph(&mut self) {
	self.end_paragraph_index = self.paragraph_index;
    }

    pub fn get_text(&self) -> String {
	let mut text = String::new();
	for i in self.paragraph_index..self.end_paragraph_index {
	    text.push_str(&self.current_text[i]);
	}
	text
    }
}
fn parse_paragraphs(reader: Reader) -> Vec<String> {
    let content = reader.current_page().unwrap().to_string();
    content.split("\n")
	   .map(|s| s.trim())
	   .filter(|s| s.starts_with("<p"))
	   .map(|s| s.to_string())
	   .collect()
}


//Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epub() {


    }
}