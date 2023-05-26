//
// Created by johan on 2023-05-02.
//

#pragma once

#include <QAbstractListModel>
#include <QQmlApplicationEngine>
#include <QQuickItem>
#include "rust/cxx.h"
#include "cxx_layer/src/client_cxx.rs.h"
class HomeModel : public QAbstractListModel {
    Q_OBJECT
private:

    rust::Vec<Library> libraries;
public:
    enum Roles {
        UuidRole = Qt::UserRole,
        NameRole,
        PathRole,
    };

    HomeModel(QObject* parent = nullptr);
    [[nodiscard]] int rowCount(const QModelIndex& parent) const override;
    int columnCount(const QModelIndex& parent) const override { return 1; }
    [[nodiscard]] QVariant data(const QModelIndex &index, int role) const override;
    [[nodiscard]] QHash<int, QByteArray> roleNames() const override;
    void updateLibraries();


public slots:
    void createLibrary(const QString& name);
    void openLibrary(int row);
    void deleteLibrary(int row);

};

