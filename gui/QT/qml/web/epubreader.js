var book = null;
var rendition = null;

function loadBook(bookPath, epubCfi) {
	book = ePub(bookPath);
	rendition = book.renderTo("area");
	if (epubCfi != null && epubCfi != "") rendition.display(epubCfi);
	else rendition.display();
	book.ready.then(function () {
		rendition.on("relocated", function(location) {
			currentCfi = location.start.cfi;
		});
	})
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
	rendition.on("relocated", function(location) {
		// Update the current cfi whenever the location is changed
		console.log(location.start.cfi);
	});
	console.log(currentCfi);
	return "hello test from";
}

function get_cfi() {
    return currentCfi;
}

function set_cfi(epubCfi) {
	rendition.display(epubCfi);
}
