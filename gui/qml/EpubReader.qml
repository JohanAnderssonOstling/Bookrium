import QtQuick 2.9
import QtQuick.Controls 2.5
import QtWebView 1.1
import QtQuick.Layouts 1.3
ColumnLayout{
	property bool showAddButton: false
	property bool showBackButton: true
	property string readerUrl
	property string bookUrl
	property string epubCfi
	property string bookUUID;
	property string bookPath;

	function loadEpub(bookPath, bookUUID){
		this.bookPath = bookPath;
		this.bookUUID = bookUUID;
		this.bookUrl = "file://" + bookPath;		
	}

	function backButtonPressed(){
		epubWebView.runJavaScript("get_cfi();", function(epubCfi){
			libraryModel.setMediaPosition(bookUUID, epubCfi);
		});
		stackView.pop();
	}

	WebView{
		id: epubWebView
		anchors.fill: parent
		url: "file:///home/johan/CLionProjects/OSPP_Project/gui/qml/web/epubreader.html"

		onLoadingChanged: {
			if (loadRequest.status !== WebView.LoadSucceededStatus) return;
			let init_cfi = libraryModel.getMediaPosition(bookUUID);
			epubWebView.runJavaScript("loadBook(\"" + bookUrl + "\", \"" + init_cfi + "\");");			//let function_call = "set_cfi(\"" + init_cfi + "\");"
			//epubWebView.runJavaScript(function_call);
		}

		Keys.onRightPressed: {
			console.log("Right pressed");
			if (event.modifiers & Qt.ControlModifier) epubWebView.runJavaScript("nextChapter();");
			else epubWebView.runJavaScript("nextPage();", function(result) {
				console.log(Result);
			});
			event.accepted = true;
		}
		Keys.onLeftPressed: {
			if (event.modifiers & Qt.ControlModifier) epubWebView.runJavaScript("prevChapter();");
			else epubWebView.runJavaScript("prevPage()");
			event.accepted = true;
		}
	}
}
