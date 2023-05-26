import QtQuick 2.9
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3

Column{
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
					var libraryViewComponent = Qt.createComponent("LibraryView.qml");
					console.log("Creating component");
					if (libraryViewComponent.status == Component.Ready){
					    console.log("Component ready");
                        var libraryView = libraryViewComponent.createObject(parent);
                        console.log("Created object with uuid" + model.uuid + " and path " + model.path);
                        libraryView.initLibraryModel(model.uuid, model.path);
                        stackView.push(libraryView);
                    }
                    console.log("Pushing library view");
                    console.log("Component status: " + libraryViewComponent.status);
                    console.log("Component errorString: " + libraryViewComponent.errorString());
				}
				else if(mouse.button & Qt.RightButton){
				    HomeModel.deleteLibrary(model.uuid);
			}}
		}
	}
	Label{
		anchors.horizontalCenter: parent.horizontalCenter
		elide: Text.ElideRight
		text: name
	}
}