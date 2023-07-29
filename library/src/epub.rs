

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

    pub fn next_chapter(&mut self) {
	let epub = rbook::Epub::new(&self.path).unwrap();
	let mut reader = epub.reader();
	self.chapter_index += 1;
	self.paragraph_index = 0;
	self.end_paragraph_index = 0;
	reader.set_current_page(self.chapter_index.clone()).unwrap().unwrap();
	self.current_text = parse_paragraphs(reader);
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

    pub fn remove_paragraph(&mut self) {
	if self.end_paragraph_index > self.paragraph_index {
	    self.end_paragraph_index -= 1;
    	}
    }

    pub fn next_paragraphs(&mut self) {
	if self.end_paragraph_index >= self.current_text.len() - 1 {
	    println!("End of chapter")	;
	    return self.next_chapter();
	}
	self.paragraph_index = self.end_paragraph_index;
    }

    pub fn reset_paragraph(&mut self) {
	self.end_paragraph_index = self.paragraph_index;
    }

    pub fn get_text(&self) -> String {
	//Get all paragraphs between paragraph_index and end_paragraph_index
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