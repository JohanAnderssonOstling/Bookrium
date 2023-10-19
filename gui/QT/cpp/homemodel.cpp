#include "homemodel.h"
#include "cxx_layer/src/client_cxx.rs.h"
#include "rust_util.h"

//Utils


HomeModel::HomeModel(QObject* parent) : QAbstractListModel {parent}
{
    start_db();
    updateLibraries();
}

QVector<QString> getCovers(const QString& path) {
	rust::Vec<rust::String> rust_covers = get_covers(asRustStr(path));
	QVector<QString> covers;
	for (int i = 0; i < 4; i++) {
		qInfo() << asQStr(rust_covers.at(i)) << Qt::endl;
		covers.append(asQStr(rust_covers.at(i)));
	}
	return covers;
}

//Model methods
QHash<int, QByteArray> HomeModel::roleNames() const {
    return { {UuidRole, "uuid"}, {NameRole, "name"}, {PathRole, "path"},
			 {CoversRole, "covers"}};
}

QVariant HomeModel::data(const QModelIndex &index, int role) const {
    Library library = this->libraries.at(index.row());
    switch (role){
        case UuidRole: return asQStr(library.uuid);
        case NameRole: return asQStr(library.name);
        case PathRole: return asQStr(library.path);
		case CoversRole: {
			rust::Vec<rust::String> rust_covers = get_covers(library.path);
			QVector<QString> covers;
			for (int i = 0; i < 4; i++) {
				qInfo() << asQStr(rust_covers.at(0)) << Qt::endl;
				covers.append(asQStr(rust_covers.at(i)));
			}
			return covers;
		}
        default: return {};
    }
}

int HomeModel::rowCount(const QModelIndex &parent) const {
    return this->libraries.size();
}

//Signals
void HomeModel::createLibrary(QString path){
    path = path.replace("file://", "");
    rust::String name = path.split("/").last().toUtf8().constData();
    create_library(name, path.toStdString(), "localhost:8080");
    updateLibraries();
}

void HomeModel::updateLibraries() {
    beginResetModel();
    this->libraries = get_libraries();
    endResetModel();
}

void HomeModel::deleteLibrary(const QString &uuid) {
    delete_library(asRustStr(uuid));
    updateLibraries();
}

void HomeModel::openLibrary(int row) {

}


