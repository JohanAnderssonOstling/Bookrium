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
	width: 200
	height: 300
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
		}}
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