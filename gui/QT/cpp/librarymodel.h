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
	rust::String containerType;
    rust::Vec<CXXBook> books;
    rust::Vec<Dir> dirs;
    QStack<Dir> dir_stack;


    QVariant bookData(int row, int role) const;
    QVariant dirData(int row, int role) const;

public:
    enum Roles {
	UUID = Qt::UserRole,
	Name,
	Path,
	HasCover,
	Cover
    };

    explicit LibraryModel(QObject *parent = 0);
    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role) const override;
    QHash<int, QByteArray> roleNames() const override;
public slots:
    void scanLibrary();
    void updateMediaFiles();
    void openLibrary(const QString &path, const QString &uuid);
    void setMediaPosition(const QString &uuid, const QString &location);
    QString getMediaPosition(const QString &uuid);
    void enterDir(int index);
    bool prevDir();
};