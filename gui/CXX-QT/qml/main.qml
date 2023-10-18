import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

// This must match the qml_uri and qml_version
// specified with the #[cxx_qt::qobject] macro in Rust.


Window {
    height: 480
    title: qsTr("Hello World")
    visible: true
    width: 640



    Column {
	anchors.fill: parent
	anchors.margins: 10
	spacing: 10

	Label {
	    text: "Number: " + myObject.number
	}

	Label {
	    text: "String: " + myObject.string
	}

	Button {
	    text: "Increment Number"

	    onClicked: myObject.incrementNumber()
	}

	Button {
	    text: "Say Hi!"

	    onClicked: myObject.sayHi(myObject.string, myObject.number)
	}
    }
}
