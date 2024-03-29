import QtQuick 2.9
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
import QtQuick.Dialogs

ColumnLayout {

    property bool showAddButton: true
    property bool showBackButton: false

    property string title: "Home"
    function addButtonPressed(){
	createLibraryDialog.open();
    }

    FolderDialog{
	id: createLibraryDialog
	title: "Select folder"
	onAccepted: {
	    console.log(createLibraryDialog.selectedFolder)
	    HomeModel.createLibrary(createLibraryDialog.selectedFolder)
	}
    }

    GridView {
	id: homeGrid
	Layout.fillWidth: true
	Layout.fillHeight: true
	cellWidth: 256 * 2 + 50
	cellHeight: 256 * 1.6 + 100
	clip: true

	model: HomeModel
	highlight: Rectangle { color: "black" }
	delegate:
	    HomeViewDelegate{}

	Keys.onDeletePressed:{
	    HomeModel.deleteLibrary(homeGrid.currentIndex)
	}
	Keys.onReturnPressed: {
	    LibraryModel.setLibrary(HomeModel.getLibrary(homeGrid.currentIndex))
	    stackView.push(libraryView)
	}
    }
    Text{
	text: homeGrid.focus ? "focus" : "not focus"
    }

}