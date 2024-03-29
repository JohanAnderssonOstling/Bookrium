import QtQuick 2.0
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
import Qt.labs.platform 1.1
import johandost.LibraryModel 1.0

ColumnLayout {
	property bool showAddButton: true
	property bool showBackButton: true
	property string title: "Library"


	function initLibraryModel(uuid, path) {
		libraryModel.openLibrary(uuid, path)
	}

	function backButtonPressed() {
		if (!libraryModel.prevDir()) stackView.pop();
	}

	function addButtonPressed() {
		newBooksDialog.open();
	}

	FileDialog {
		id: newBooksDialog
		title: "Select books"
		folder: StandardPaths.writableLocation(StandardPaths.DocumentsLocation)
		//selectMultiple: true
		fileMode: FileDialog.OpenFiles
		nameFilters: "Books (*.epub *pdf)"
		onAccepted: {
			LibraryModel.addBooks(newBooksDialog.files)
		}
	}

	RowLayout {
		Button {
			text: "Scan"
			onClicked: {
				libraryModel.scanLibrary()
			}
		}
		Button {
			text: "Update Media"
			onClicked: {
				libraryModel.updateMediaFiles()
			}
		}
		Button {
			text: "Home"
		}
		Button {
			text: "Books"
		}
		Button {
			text: "Authors"
		}
		Button {
			text: "Folders"
		}
		Button {
			text: "Subjects"
		}
		Button {
			text: "Publishers"
		}
	}

	RowLayout {
		GridView {
			id: 	libraryGrid
			model: 	LibraryModel 	{id: libraryModel}
			Layout.fillWidth:  	true
			Layout.fillHeight: 	true
			property int coverWidth: 	256
			cellWidth: 	coverWidth + 24
			cellHeight: coverWidth * 1.6 + 60
			clip: 	true
			ScrollBar.vertical: ScrollBar {}

			delegate: LibraryViewDelegate {
				coverWidth: libraryGrid.coverWidth
				coverHeight: libraryGrid.coverWidth * 1.6
			}

		}
	}

}