import QtQuick
import QtQuick.Controls
import QtQuick.Layouts


Window {
	id: window
	width: 640
	height: 480
	visible: true
	title: qsTr("Juveler")

	ColumnLayout{
		anchors.fill:parent
		ToolBar{
			Layout.alignment: Qt.AlignTop
			Layout.fillWidth: true
			RowLayout{
				anchors.fill: parent
				ToolButton{
					visible: stackView.currentItem.showBackButton
					text: "<"
					focus: false
					onClicked: stackView.currentItem.backButtonPressed();
				}
				Label{
					text: stackView.currentItem.title
					elide: Label.ElideRight
					Layout.alignment: Qt.AlignCenter
				}
				ToolButton{
					visible: stackView.currentItem.showAddButton;
					text: "+"
					Layout.alignment: Qt.AlignRight
					onClicked: stackView.currentItem.addButtonPressed();
				}
			}
		}
	StackView{
		id: stackView
		Layout.fillWidth: true
		Layout.fillHeight: true
		focus: true
		HomeView{id: homeView}
		initialItem: homeView
		}

	}

}