import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import johandost.EpubModel 1.0
Row {

    function backButtonPressed() {
	stackView.pop();
    }

    property var loaded: false
    property var uuid;
    property var epubPath;
    property string title;
    EpubModel { id: epubModel }


    readonly property int maxCharacters: 80
    property font textEditFont: Qt.font({
	pointSize: 16 // Or any desired font size
    })

    FontMetrics {
	id: fontMetrics
	font: textEditFont
    }

    property real textEditMaxWidth: 80 * fontMetrics.averageCharacterWidth

    readonly property int textEditCount: Math.max(1, Math.floor(parent.width / textEditMaxWidth))
    Component.onCompleted: {
	layout();
    }

    Repeater {
	id: epubReader
	model: textEditCount
	onModelChanged: {
	    console.log("model changed");
	    layout();
	}

	TextEdit {
	    width: parent.width / textEditCount
	    height: parent.height
	    textFormat: Text.RichText
	    wrapMode: TextEdit.WordWrap
	    selectByMouse: true
	    font: textEditFont
	    readOnly: true
	    onLinkActivated: {
		epubModel.goTo(link);
		layout();
	    }

	    onLinkHovered: {}
	}
    }

    Keys.onLeftPressed: {
	if (event.modifiers && Qt.ControlModifier) {
	    epubModel.prevChapter();
	    console.log(textEditMaxWidth);
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

    Keys.onPressed: {
	if (event.key === Qt.Key_Plus) {
	    // Handle "+" key press here
	    textEditFont.pointSize += 1
	    layout();
	} else if (event.key === Qt.Key_Minus) {
	    // Handle "-" key press here
	    textEditFont.pointSize -= 1
	    layout();
	}
    }

    onWidthChanged: layout()

    function loadEpub (uuid, epubPath, title) {
	this.uuid = uuid;
	this.epubPath = epubPath;
	this.title = title;
	epubModel.openEpub(epubPath);
	epubModel.setPos(libraryModel.getMediaPosition(uuid));
	loaded = true;
	layout();
    }

    function layout() {
	if (!loaded) return;
	epubModel.resetParagraph();
	libraryModel.setMediaPosition(uuid, epubModel.getPos());
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

