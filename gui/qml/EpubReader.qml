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
	property string title

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
			let init_cfi = libraryModel.getMediaPosition(bookUUID);
			let function_call = "set_cfi(\"" + init_cfi + "\");"
			epubWebView.runJavaScript(function_call);
		}

		Keys.onRightPressed: epubWebView.runJavaScript("nextPage();");
		Keys.onLeftPressed: epubWebView.runJavaScript("prevPage()");
		}
}
