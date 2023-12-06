import QtQuick 2.9
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3

Column{
    Menu {
	id: homeContextMenu
	MenuItem {
	    text: "Delete"
	    onTriggered: {
		HomeModel.deleteLibrary(uuid);
	    }
	}
    }
    Rectangle{
	anchors.horizontalCenter: parent.horizontalCenter
	width: 256 * 2
	height: 400
	color: "lightblue"
	MouseArea{
	    anchors.fill: parent
	    acceptedButtons: Qt.LeftButton | Qt.RightButton
	    onClicked: {
		if (mouse.button & Qt.LeftButton){
		    openLibrary();
		}
		else if(mouse.button & Qt.RightButton){
		    homeContextMenu.popup()
		}
	    }
	    Row {
		spacing: 0
		anchors.bottom: parent.bottom
		Image {
		    source: "file://" + covers[0]
		    anchors.bottom: parent.bottom
			width:256
			height:400
		}
		Image {
		    source: "file://" + covers[1]
		    anchors.bottom: parent.bottom
			width:256
			height:400
		}
	    }
	}
    }
    Label{
	anchors.horizontalCenter: parent.horizontalCenter
	elide: Text.ElideRight
	text: name
    }

    function openLibrary () {
	var libraryViewComponent = Qt.createComponent("LibraryView.qml");
	if (libraryViewComponent.status == Component.Ready){
	    var libraryView = libraryViewComponent.createObject(parent);
	    libraryView.initLibraryModel(model.uuid, model.path);
	    stackView.push(libraryView);
	}
    }
}