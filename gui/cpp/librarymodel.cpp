//
// Created by johan on 2023-05-02.
//
#include "librarymodel.h"

/*
#include "librarymodel.h"
*/

#include "rust_util.h"

LibraryModel::LibraryModel(QObject *parent) : QAbstractListModel(parent) {
}

void LibraryModel::openLibrary(const QString &uuid, const QString &path) {
	this->library_path = asRustStr(path);
	this->library_uuid = asRustStr(uuid);

	open_library(this->library_uuid);
	updateMediaFiles();
}

QString get_media_name(const MediaFile &mediaFile) {
	QString title = asQStr(mediaFile.title);
	if (title.isEmpty()) title = asQStr(mediaFile.path).split("/").last();
	return title;
}

QVariant LibraryModel::data(const QModelIndex &index, int role) const {
	const MediaFile mediaFile = media_files.at(index.row());

	switch (role) {
		case UUIDRole: return asQStr(mediaFile.uuid);
		case PathRole: return asQStr(mediaFile.path);
		case NameRole: return get_media_name(mediaFile);
		case HasCoverRole: return has_cover(this->library_uuid, mediaFile.uuid);
		case CoverRole:
			return asQStr(get_cover_path(this->library_uuid, mediaFile.uuid));
	}
}


int LibraryModel::rowCount(const QModelIndex &parent) const {
	return this->media_files.size();
}

int LibraryModel::columnCount(const QModelIndex &parent) const { return 1; }

QHash<int, QByteArray> LibraryModel::roleNames() const {
	return {{UUIDRole, "uuid"},
					{NameRole, "name"},
					{PathRole, "path"},
					{HasCoverRole, "hasCover"},
					{CoverRole, "cover"}};
}

void LibraryModel::scanLibrary() {
	scan_library(this->library_uuid, this->library_path);
	updateMediaFiles();
}

void LibraryModel::updateMediaFiles() {
	beginResetModel();
	this->media_files = get_media_files(this->library_uuid);
	endResetModel();
}

void
LibraryModel::setMediaPosition(const QString &uuid, const QString &location) {
	qInfo() << "setMediaPosition" << uuid << location;
	set_media_position(this->library_uuid, asRustStr(uuid), asRustStr(location));
}

QString LibraryModel::getMediaPosition(const QString &uuid) {
	QString pos = asQStr(get_media_position(this->library_uuid, asRustStr(uuid)));
	qInfo() << "getMediaPosition" << uuid << pos;
	return pos;
}

