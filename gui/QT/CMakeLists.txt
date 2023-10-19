cmake_minimum_required(VERSION 3.16)
project(Castafiore VERSION 0.1 LANGUAGES CXX)


set(CMAKE_CXX_STANDARD 17)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS true)


find_package(Qt6 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner WebView)

find_package(Corrosion REQUIRED)
include(corrosion_cxx.cmake)

qt_add_resources(RESOURCES qml/qml.qrc)

qt_add_executable(Castafiore
        cpp/main.cpp cpp/homemodel.cpp cpp/librarymodel.cpp
        cpp/homemodel.h cpp/librarymodel.h cpp/rust_util.h
        qml/main.qml qml/HomeView.qml qml/HomeViewDelegate.qml
        qml/LibraryView.qml qml/LibraryViewDelegate.qml qml/PDFReader.qml qml/EpubReader.qml
        cpp/epubmodel.h
        cpp/epubmodel.cpp
        cpp/tocmodel.cpp
        cpp/tocmodel.h)

add_library_rust(PATH rust/cxx_layer NAMESPACE castafiore_cxx)
target_link_libraries(Castafiore
        PUBLIC castafiore_cxx::cxx_layer
        PRIVATE Qt6::Quick Qt6::Core Qt6::WebView ssl bz2)
