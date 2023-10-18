import QtQuick 2.9
import QtQuick.Controls 2.5
import QtWebView 1.1
import QtQuick.Layouts 1.3
import johandost.TocModel 1.0
RowLayout {
    property bool showAddButton: false
    property bool showBackButton: true
    property string bookUrl
    property string epubCfi
    property string bookUUID;
    property string title: "Epub Reader"
    property string libraryUUID
    property int bookProgress: 0

    function loadEpub(bookPath, bookUUID, libraryUUID) {
	this.bookUUID = bookUUID;
	this.bookUrl = "file://" + bookPath;
	tocModel.setToc(libraryModel.getLibraryUuid(), bookUUID);
    }

    function backButtonPressed() {
	epubWebView.runJavaScript("get_cfi();", function (epubCfi) {
	    libraryModel.setMediaPosition(bookUUID, epubCfi, bookProgress);
	});
	stackView.pop();
    }

    ListView {
	width: 200
	Layout.fillHeight: true
	model: TocModel {id : tocModel; }
	delegate: Component {
	    Item {
		width: parent.width
		height: nameText.height + 10
		Text {
		    id: nameText
		    text: model.name
		    font.pixelSize: 16
		    color: "blue"
		    MouseArea {
			id: mouseArea
			anchors.fill: parent
			cursorShape: Qt.PointingHandCursor
			onClicked: {
			    set_cfi(model.href);
			}
		    }
		}
	    }
	}
    }

    ColumnLayout{
	WebView {
	    id: epubWebView
	    Layout.fillWidth: true
	    Layout.fillHeight: true
	    url: Qt.resolvedUrl("web/epubreader.html")

	    onLoadingChanged: {
		console.log("loading changed");
		if (loadRequest.status !== WebView.LoadSucceededStatus) return;
		let init_cfi = libraryModel.getMediaPosition(bookUUID);
		epubWebView.runJavaScript("loadBook(\"" + bookUrl + "\", \"" + init_cfi + "\");");			//let function_call = "set_cfi(\"" + init_cfi + "\");"
	    	updateReadingProgress();
	    }

	    Keys.onRightPressed: {
		if (event.modifiers && Qt.ControlModifier)
		    epubWebView.runJavaScript("nextChapter();");
		else epubWebView.runJavaScript("nextPage();");
		event.accepted = true;
		updateReadingProgress();
	    }

	    Keys.onLeftPressed: {
		if (event.modifiers & Qt.ControlModifier)
		    epubWebView.runJavaScript("prevChapter();");
		else epubWebView.runJavaScript("prevPage()");
		event.accepted = true;
		updateReadingProgress();
	    }
	}
	RowLayout {
	    Label {

	    }
	    Label {

	    }
	}

    }

    function updateReadingProgress() {
	epubWebView.runJavaScript("getReadingProgress();", function (currentProgress) {
	    console.log("currentProgress: " + currentProgress);
	    bookProgress = currentProgress;
	});
	var cfi = "";
	epubWebView.runJavaScript("get_cfi();", function (epubCfi) {
	    cfi = epubCfi;
	});
	console.log("updateReadingProgress: " + cfi + " " + bookProgress);
	libraryModel.setMediaPosition(bookUUID, cfi, bookProgress);
    }


    function set_cfi(cfi) {
	epubWebView.runJavaScript("set_cfi(\"" + cfi + "\");");
	updateReadingProgress();
    }
}
