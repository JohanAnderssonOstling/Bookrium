let book = null;
let rendition = null;
let currentCfi = null;
let currentProgress = 0.0;
//loadBook("file:///home/johan/Hem/Downloads/PDFBöcker/[9781732102200] John Ousterhout - A Philosophy of Software Design.epub", "")
function loadBook(bookPath, epubCfi) {
	book = ePub(bookPath);
	rendition = book.renderTo("area");
	if (epubCfi != null && epubCfi !== "") rendition.display(epubCfi);
	else rendition.display();
	book.ready.then(function () {
		return book.locations.generate(1600);
	}).then(function(locations){
		rendition.on("relocated", function(location) {
			currentCfi = location.start.cfi;
			currentProgress = rendition.location.start.percentage;
			console.log(currentProgress)
		});

	});
	nextChapter();
}

function prevChapter() {
	changeChapter(-1);
}

function changeChapter(delta) {
	const currentLocation = book.rendition.currentLocation();
	let newChapter = book.spine.spineItems.findIndex(item => item.href === currentLocation.start.href) + delta;
	
	if (newChapter != 0 && newChapter < book.spine.spineItems.length) {
		let chapter = book.spine.spineItems[newChapter];
		rendition.display(chapter.href);
	}
} 

function nextChapter() {
	changeChapter(1);
}

function prevPage() {
    rendition.prev();
}

function nextPage() {
    rendition.next();
}

function getReadingProgress() {
	return Math.floor(book.locations.percentageFromCfi(currentCfi) * 100);
}
function get_cfi() {
    return currentCfi;
}

function set_cfi(epubCfi) {
	rendition.display(epubCfi);
}
