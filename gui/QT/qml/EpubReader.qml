import QtQuick 2.9
import QtQuick.Controls 2.5
import QtWebView 1.1
import QtQuick.Layouts 1.3

ColumnLayout {
    property bool showAddButton: false
    property bool showBackButton: true
    property string bookUrl
    property string epubCfi
    property string bookUUID;
    property string title: "Epub Reader"

    function loadEpub(bookPath, bookUUID) {
	this.bookUUID = bookUUID;
	this.bookUrl = "file://" + bookPath;
    }

    function backButtonPressed() {
	epubWebView.runJavaScript("get_cfi();", function (epubCfi) {
	    libraryModel.setMediaPosition(bookUUID, epubCfi);
	    console.log("qrc:/epubreader.html");
	});
	stackView.pop();
    }

    WebView {
	id: epubWebView
	Layout.fillWidth: true
	Layout.fillHeight: true
	url: Qt.resolvedUrl("web/epubreader.html")

	onLoadingChanged: {
	    if (loadRequest.status !== WebView.LoadSucceededStatus) return;
	    let init_cfi = libraryModel.getMediaPosition(bookUUID);
	    console.log("Loading book: " + bookUrl + " with cfi: " + init_cfi);
	    console.log(Qt.resolvedUrl("web/epubreader.html") + "\t" + epubWebView.url);
	    epubWebView.runJavaScript("loadBook(\"" + bookUrl + "\", \"" + init_cfi + "\");");			//let function_call = "set_cfi(\"" + init_cfi + "\");"
	}

	Keys.onRightPressed: {
	    if (event.modifiers && Qt.ControlModifier)
		epubWebView.runJavaScript("nextChapter();");
	    else epubWebView.runJavaScript("nextPage();");
	    event.accepted = true;
	}

	Keys.onLeftPressed: {
	    if (event.modifiers & Qt.ControlModifier)
		epubWebView.runJavaScript("prevChapter();");
	    else epubWebView.runJavaScript("prevPage()");
	    event.accepted = true;
	}
    }
}
