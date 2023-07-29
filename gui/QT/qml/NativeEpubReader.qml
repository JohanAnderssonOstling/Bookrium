import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import johandost.EpubModel 1.0
RowLayout {
    property var loaded: false
    EpubModel { id: epubModel }

    Repeater {
	id: epubReader
	model: 4
	TextEdit {
	    Layout.preferredWidth: parent.width / 4
	    Layout.preferredHeight: parent.height
	    textFormat: Text.RichText
	    wrapMode: TextEdit.WordWrap
	    selectByMouse: true
	    readOnly: true
	    onLinkActivated: {
		epubModel.goTo(link);
		layout();
	    }

	    onLinkHovered: {

	    }
	    text: "5"
	}
    }

    Keys.onLeftPressed: {
	if (event.modifiers && Qt.ControlModifier) {
	    epubModel.prevChapter();
	    layout();
	    return;
	}
	epubModel.prevParagraphs();
	layoutReverse();
	layout();
    }

    Keys.onRightPressed: {
	if (event.modifiers && Qt.ControlModifier) {
	    epubModel.nextChapter();
	    layout();
	    return;
	}
	epubModel.nextParagraphs();
	layout();
    }



    Component.onCompleted: {
	console.log("Opening epub");
	epubModel.openEpub("");
	epubModel.nextChapter();
	epubModel.nextChapter();
	epubModel.nextChapter();
	epubModel.nextChapter();
	epubModel.nextChapter();
	loaded = true;
	console.log("Loaded: " + loaded );
    }
    onWidthChanged: layout()

    function layout() {
	if (!loaded) return;
	epubModel.resetParagraph();
	resetText();
	for (let i = 0; i < epubReader.count; i++) {
	    var item = epubReader.itemAt(i);
	    var oldText = "";
	    item.text = "";
	    while (item.height > item.contentHeight) {
		oldText = item.text;
		let newText = epubModel.addParagraph();
		if (newText === "EOF") return;
		item.text = oldText + newText;
	    }
	    item.text = oldText;
	    epubModel.removeParagraph();
	}
    }

    function layoutReverse () {
	if (!loaded) return;
	epubModel.resetParagraph();
	resetText();
	for (let i = epubReader.count - 1; i >= 0; i--) {
	    var item = epubReader.itemAt(i);
	    var oldText = "";
	    item.text = "";
	    while (item.height > item.contentHeight) {
		oldText = item.text;
		let newText = epubModel.addPrevParagraph();
		if (newText === "BOF") return;
		item.text = newText + oldText;
	    }
	    item.text = oldText;
	    epubModel.removePrevParagraph();
	}
    }

    function resetText () {
	for (let i = 0; i < epubReader.count; i++)
	   epubReader.itemAt(i).text = "";
    }
}

