let book = null;
let rendition = null;
let currentCfi = null;
console.log("epubreader.js loaded");
alert("epubreader.js loaded");
loadBook("file:///home/johan/Hem/Downloads/PDFBöcker/[9781732102200] John Ousterhout - A Philosophy of Software Design.epub", "")
function loadBook(bookPath, epubCfi) {
	alert("Loading book" + bookPath);
	console.log("Loading book: " + bookPath);
	book = ePub(bookPath);
	rendition = book.renderTo("area");
	alert(rendition);
	if (epubCfi != null && epubCfi !== "") rendition.display(epubCfi);
	else rendition.display();
	book.ready.then(function () {
		alert("Book ready");
		rendition.on("relocated", function(location) {
			currentCfi = location.start.cfi;
		});
	})
	alert("Book loaded");
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
	alert("Next chapter");
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
