//
// Created by johan on 2023-05-02.
//

#pragma once

#include <qt6/QtCore/QAbstractListModel>
#include <qt6/QtCore/QStack>
#include "rust/cxx.h"
#include "cxx_layer/src/library_cxx.rs.h"
#include "cxx_layer/src/client_cxx.rs.h"

class LibraryModel : public QAbstractListModel {
Q_OBJECT
private:

    rust::String library_uuid;
    rust::String library_path;
    rust::Vec<MediaFile> media_files;
    //QStack<int> navStack;

public:
    enum Roles {
        UUIDRole = Qt::UserRole,
        NameRole,
        PathRole,
        AuthRole,
        LocationRole,
        HasCoverRole,
        CoverRole
    };

    explicit LibraryModel(QObject *parent = 0);
    int columnCount(const QModelIndex& parent = QModelIndex()) const override;
    int rowCount(const QModelIndex& parent = QModelIndex()) const override;
    [[nodiscard]] QVariant data(const QModelIndex &index, int role) const override;
    [[nodiscard]] QHash<int, QByteArray> roleNames() const override;

public slots:
    void scanLibrary();
    void updateMediaFiles();

    void openLibrary(const QString& path, const QString& uuid);
};