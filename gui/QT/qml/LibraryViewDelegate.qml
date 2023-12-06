import QtQuick 2.9
import QtQuick.Controls 2.5
import QtQuick.Layouts
import johandost.LibraryModel
import QtQuick.Pdf
Column {
	id: column
	property int coverWidth
	property int coverHeight
	width:coverWidth
	height: coverHeight
	Layout.fillWidth: true

	Menu {
		id: libraryContextMenu
		MenuItem {
			text: "Delete"
			onTriggered: {
				if (isContainer && (result =libraryModel.deleteDir(uuid)) !== "") 	console.log("error deleting dir: " + result);
				else if ((result = libraryModel.deleteBook(uuid)) !== "") 			console.log("error deleting book: " + result);
			}
		}
	}
	Rectangle {
		id: bookCoverContainer
		width: coverWidth
		height: coverHeight
		//clip: true
		MouseArea {
			anchors.fill: parent
			hoverEnabled: true
			acceptedButtons: Qt.LeftButton | Qt.RightButton
			onReleased: {
				if (mouse.button === Qt.LeftButton) openMedia();
				else libraryContextMenu.popup();
			}

			onEntered: hoverEnter()
			onExited:  hoverExit()
		}
		Image {
			id: bookCover
			visible: hasCover
			source: "file://" + cover
			asynchronous: true
			anchors.bottom: bookCoverContainer.bottom
			//anchors.centerIn: parent
			clip: true
			Rectangle {
				id: bookCoverFilter
				anchors.fill: parent
				color: "transparent"
				opacity: 0.25
				border.color: "black"
				border.width: 1

			}
		}
		ProgressBar {
			id: bookProgress
			width: bookCoverContainer.width
			height: 5 // Adjust the height as needed
			anchors.bottom: bookCoverContainer.bottom
			value: progress / 100 // Example value, set this to your actual progress value
			visible: !isContainer && progress != 0
		}
	}

	function hoverEnter() {
		bookCoverFilter.color = "black"
	}
	function hoverExit() {
		bookCoverFilter.color = "transparent"
	}

	Text {
		width:parent.width;	height:60
		text:name
		elide:Text.ElideRight;	wrapMode:Text.Wrap
		horizontalAlignment: 	Text.AlignHCenter
	}


	function openMedia() {
		console.log("path: " + path)
		let split_path = path.split(".");
		if (split_path.length === 1) return libraryModel.enterDir(index);
		let extension = split_path.pop();
		if (extension === "pdf") return openPdf();
		if (extension === "epub") return openEpub();
	}

	function openPdf() {
		let pdfReaderComp = Qt.createComponent("PDFReader.qml");
		if (pdfReaderComp.status === Component.Ready) {
			let pdfReader = pdfReaderComp.createObject(parent);
			pdfReader.documentSource = "file://" + path
			pdfReader.title = name + " uuid: " + uuid
			pdfReader.uuid = uuid
			pdfReader.init_read_location = 0
			stackView.push(pdfReader);
		}
		else {
			console.log("error loading component");
			console.log(pdfReaderComp.errorString());
		}
	}

	function openEpub() {
		let epubReaderComp = Qt.createComponent("EpubReader.qml");
		if (epubReaderComp.status === Component.Ready) {
			let epubReader = epubReaderComp.createObject(parent);
			epubReader.loadEpub(path, uuid);
			stackView.push(epubReader);
		} else {
			console.log("error loading component");
			console.log(epubReaderComp.errorString());
		}
	}
	function openNativeEpub() {
		let NativeEpubReader = Qt.createComponent("NativeEpubReader.qml");
		let nativeEpubReader = NativeEpubReader.createObject(parent);
		nativeEpubReader.loadEpub(uuid, path, name);
		stackView.push(nativeEpubReader);
	}

}
