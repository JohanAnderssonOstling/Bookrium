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
	width: 300
	height: 450
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

	GridLayout {
	    columns: 2
	    anchors.centerIn: parent
	    Image {
		source: "file://" + covers[0]
	    }
	    Image {
		source: "file://" + covers[1]
	    }
	    Image {
		source: "file://" + covers[2]

	    }
	    Image {
		source: "file://" + covers[3]

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