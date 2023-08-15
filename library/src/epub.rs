use rbook::{Ebook, Reader};
use html_parser::Dom;
pub struct Epub {
    path: String,
    current_text: Vec<String>,
    chapter_index: usize,
    paragraph_index: usize,
    end_paragraph_index: usize,
}

pub struct Epub2 {
	chapters: Vec<Option<String>>,
}
pub struct Chapter {

}

impl Chapter {
	pub fn new(html: String) -> Self {
		let dom = Dom::parse(html.as_str()).unwrap();
		for node in dom.children {
			print_node(&node);

		}
		Self {
		}
	}
}

fn print_node (node: &html_parser::Node) {
	println!("{:#?}", node);
	if node.element().is_none() {
		return;
	}
	for child in node.element().unwrap().children.iter() {
		print_node(child);
	}
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

	reader.set_current_page_str(format!("text/{}", page).as_str()).unwrap().unwrap();
	self.chapter_index = reader.current_index();
	self.paragraph_index = 0;
	self.end_paragraph_index = 0;
	self.current_text = parse_paragraphs(reader);
	for (i, paragraph) in self.current_text.iter().enumerate() {
	    if paragraph.contains(format!("id=\"{}\"", id).as_str()) {
			self.paragraph_index = i;
			self.end_paragraph_index = i;
			return;
	    }
	}
    }

    pub fn next_chapter(&mut self) {

	let epub = rbook::Epub::new(&self.path).unwrap();
	let mut reader = epub.reader();
	if (reader.page_count() - 1) == self.chapter_index { return }

	self.chapter_index 		+=1;
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
	if self.paragraph_index == 0 { return "BOF".into(); }
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
	if self.paragraph_index == 0 { self.prev_chapter(); }
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

    pub fn get_pos(&self) -> String {
	format!("{}:{}", self.chapter_index, self.paragraph_index)
    }

    pub fn set_pos(&mut self, pos: &str) {
	if pos.is_empty() { return; }
	let pos = pos.split(':').collect::<Vec<&str>>();
	self.chapter_index 		= pos[0].parse::<usize>().unwrap();
	self.paragraph_index 	= pos[1].parse::<usize>().unwrap();
	self.end_paragraph_index = self.paragraph_index;

    }

   /* pub fn get_toc (&self) -> String {
	let epub = rbook::Epub::new(&self.path).unwrap();

	let toc = epub.toc().elements();
	toc.iter().for_each(|chapter| {
	    println!("{}: {}", chapter.title, chapter.href);
	});
	let mut toc_string = String::new();
	for (i, chapter) in toc.iter().enumerate() {
	    toc_string.push_str(format!("{}: {}\n", i, chapter.title).as_str());
	}
	toc_string
    }*/
}
fn parse_paragraphs(reader: Reader) -> Vec<String> {
    let content = reader.current_page().unwrap().to_string();
    content.split(". ")
	   .map(str::trim)
	   .map(str::to_owned)
	   .map(|s| format!("{s}."))
	   .collect()
}




//Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epub() {
		let input = "<h1 class=\"calibre23\" id=\"calibre_pb_27\"><span class=\"calibre24\"><span class=\"bold1\"><span class=\"bold1\">Coalition Dynamics</span></span></span></h1><div class=\"calibre22\"><span class=\"calibre10\">Lincoln, <b>more</b> than any other winner of the presidency, foresaw that he would not be popular among a vast segment of voters in the presidential election. He understood that his best chance, maybe even his only chance for election in 1860, lay in dividing and conquering. Had Douglas answered Lincoln’s question with a pro-slavery response (that is, in support of the Dredd Scott Decision as the law of the land), he almost certainly would have lost the senate race to Lincoln. That might have kept the Democrats united in 1860, but it would have boosted Lincoln’s prospects as the senate incumbent with a popular following. By answering as he did, Douglas guaranteed that his own party would divide over his presidential bid. With competitors Breckinridge and Bell contesting the presidency, Douglas lost his opportunity to win the southern vote, dooming him—and his Democratic rivals—to defeat, even though Lincoln’s vote total was slim. Lincoln beat the divided Democrats with less than 40 percent of the popular vote and almost no votes in the South. Similarly, Bill Clinton, with just 43 percent of the vote beat the incumbent President George H. W. Bush (who won 38 percent of the popular vote) in 1992, in no small measure thanks to the run by H. Ross Perot (who got 19 percent of the vote).<a class=\"calibre11\" href=\"../Text/The_Dictators_Handbook_split_094.html#filepos752049\" id=\"filepos167005\"><sup class=\"calibre25\">11</sup></a> Lincoln understood that he needed to keep the coalition as small as possible—even in an inherently large coalition system.</span></div><div class=\"calibre22\"><span class=\"calibre10\">Lincoln did not lose sight of this important principal as he sought reelection in 1864. Seeing that his prospects were not great, he maneuvered to expand the set of interchangeables and influentials so that he could forge a winning coalition out of those who previously had no say at all. How did he do this? He introduced absentee ballots so that soldiers could vote, with an especially important impact in New York. It is widely believed that the vote of soldiers carried the state for Lincoln in his 1864 race against General George B. McClellan. Lincoln was a master at using the rules of politics to his advantage, winning while being unpopular with a large swath of the American people.</span></div>";
		Chapter::new(input.to_string());
	}


}