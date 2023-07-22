#include "homemodel.h"
#include "cxx_layer/src/client_cxx.rs.h"
#include "rust_util.h"

//Utils


HomeModel::HomeModel(QObject* parent) : QAbstractListModel {parent}
{
    start_db();
    updateLibraries();
}

//Model methods
QHash<int, QByteArray> HomeModel::roleNames() const {
    return { {UuidRole, "uuid"}, {NameRole, "name"}, {PathRole, "path"} };
}

QVariant HomeModel::data(const QModelIndex &index, int role) const {
    Library library = this->libraries.at(index.row());
    switch (role){
        case UuidRole: return asQStr(library.uuid);
        case NameRole: return asQStr(library.name);
        case PathRole: return asQStr(library.path);
        default: return {};
    }
}

int HomeModel::rowCount(const QModelIndex &parent) const {
    return this->libraries.size();
}

//Signals
void HomeModel::createLibrary(const QString& path){
    rust::String name = path.split("/").last().toUtf8().constData();
    create_library(name, path.toStdString(), "localhost:8080");
    updateLibraries();
}

void HomeModel::updateLibraries() {
    beginResetModel();
    this->libraries = get_libraries();
    endResetModel();
}

void HomeModel::deleteLibrary(int row) {
    delete_library(this->libraries.at(row).uuid);
    updateLibraries();
}

void HomeModel::openLibrary(int row) {

}
