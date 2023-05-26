//
// Created by johan on 2023-05-02.
//
#include "librarymodel.h"

/*
#include "librarymodel.h"
*/

#include "rust_util.h"
LibraryModel::LibraryModel(QObject* parent):QAbstractListModel(parent) {
}

void LibraryModel::openLibrary(const QString& uuid, const QString& path) {
    this->library_path = asRustStr(path);
    this->library_uuid = asRustStr(uuid);

    open_library(this->library_uuid);
    updateMediaFiles();
}

QVariant LibraryModel::data(const QModelIndex &index, int role) const {
    const MediaFile mediaFile = media_files.at(index.row());

    switch (role) {
        case UUIDRole: return asQStr(mediaFile.uuid);
        case PathRole: return asQStr(mediaFile.path);
        case NameRole: return asQStr(mediaFile.path).split("/").last();
        case HasCoverRole: return has_cover(this->library_uuid, mediaFile.uuid);
        case CoverRole: return asQStr(get_cover_path(this->library_uuid, mediaFile.uuid));
    }
}


int LibraryModel::rowCount(const QModelIndex &parent) const {
    return this->media_files.size();
}

QHash<int, QByteArray> LibraryModel::roleNames() const {
    return {{UUIDRole, "uuid"},
            {PathRole, "path"},
            {NameRole, "name"},
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

int LibraryModel::columnCount(const QModelIndex &parent) const {
    Q_UNUSED(parent);
    return 1;
}

