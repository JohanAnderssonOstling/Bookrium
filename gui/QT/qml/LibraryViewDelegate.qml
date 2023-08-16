import QtQuick 2.0
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
import Qt.labs.platform 1.1
import johandost.LibraryModel 1.0
import QtQuick.Pdf

Column {
    id: column
    property int coverWidth
    property int coverHeight
    property string library_uuid
    width:coverWidth
    height: coverHeight
    Layout.fillWidth: true


    Image {
	id: 		bookCover
	visible: 	hasCover
	source: 	"file://" + cover
	asynchronous: 	true
	MouseArea {
	    anchors.fill: parent
	    onClicked: openMedia()
	}
    }

    /*Rectangle {
	id: libraryFolder
	visible: !hasCover
	width: coverWidth
	height: coverHeight
	color: "blue"
	MouseArea {
	    anchors.fill: parent
	    onClicked: openDir()
	}
    }*/

    Text {
	width:parent.width;	height:40
	text:name
	elide:Text.ElideRight;	wrapMode:Text.Wrap

	horizontalAlignment: 	Text.AlignHCenter
    }

    function openDir() {
	console.log("openDir")
	libraryModel.enterDir(index)
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
	    epubReader.loadEpub(path, uuid, library_uuid);
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
