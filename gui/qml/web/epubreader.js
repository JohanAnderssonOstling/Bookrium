let bookUrl = "file:///home/johan/Hem/Downloads/[9781732102200] John Ousterhout - A Philosophy of Software Design (0) - libgen.li.epub"
let book = ePub(bookUrl);
let rendition = book.renderTo("area");
rendition.display();

let currentCfi = null;
book.ready.then(function () {
	rendition.on("relocated", function(location) {
		// Update the current cfi whenever the location is changed
		currentCfi = location.start.cfi;
	});
})

function prevPage() {
    rendition.prev();
}

function nextPage() {
    rendition.next();
}



function get_cfi() {
    return currentCfi;
}

function set_cfi(epubCfi) {
	rendition.display(epubCfi);
}
