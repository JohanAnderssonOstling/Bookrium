import QtQuick
import QtQuick.Pdf
import QtQuick.Layouts
ColumnLayout{
	property bool showAddButton: false
	property bool showBackButton: true
	function backButtonPressed(){
		stackView.pop();
	}
	property var title
	property url documentSource
	property string uuid
	property var init_read_location

	PdfPageView {
		id : pdfPageView
		document: PdfDocument{ id: pdfDocument; source: documentSource;}

		Layout.alignment: Qt.AlignCenter
		Layout.fillWidth: true
		Layout.fillHeight: true
		Layout.margins: 20

		property bool first: false
		onStatusChanged: {
			pdfPageView.focus = true;
			focus = true;
			scaleToPage(width, height);
			if (status == 1 && !first){
				first = true;
				console.log("Loaded book: " + name + " at position " + init_read_location);
				let init_page = libraryModel.getMediaPosition(uuid);
				let page = parseInt(init_page);
				console.log("Seting page to" + page);
				setPage(page);
			}
		}
		focus: true
		Keys.onPressed: pdfPageView.keyPress(event)

		function keyPress(event){
			switch(event.key){
				case Qt.Key_Right: pdfPageView.nextPage(); break;
				case Qt.Key_Left: pdfPageView.previousPage(); break;
			}
		}

		function nextPage(){pdfPageView.changePage(1);}
		function previousPage(){pdfPageView.changePage(-1);}

		function changePage(delta){
			var newPage = pdfPageView.currentPage + delta;
			setPage(newPage);
		}

		function setPage(newPage){
			let pageCount = pdfPageView.document.pageCount;
			if(newPage < 0 || newPage >= pageCount){ console.log("Page out of range: " + newPage); return;}
			pdfPageView.goToPage(newPage);
			libraryModel.setMediaPosition(uuid, newPage.toString());
		}

	}
	Text{
		text: pdfPageView.currentPage
	}

}



