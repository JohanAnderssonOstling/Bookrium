//
// Created by johan on 2023-08-16.
//

#ifndef CASTAFIORE_TOCMODEL_H
#define CASTAFIORE_TOCMODEL_H


#include <QStandardItemModel>
#include "rust/cxx.h"
#include "cxx_layer/src/library_cxx.rs.h"
class TocModel : public QAbstractListModel{
	Q_OBJECT
private:
	rust::Vec<Nav> toc;
public:
	enum Roles {
		Name = Qt::UserRole + 1,
		Href,
	};

	QVariant data(const QModelIndex &index, int role) const override;
	QHash<int, QByteArray> roleNames() const override;
	int rowCount(const QModelIndex &parent) const override { return toc.size(); }
	int columnCount(const QModelIndex &parent) const override { return 1; }
	public slots:
	void setToc(QString library_uuid, QString book_uuid);
};


#endif //CASTAFIORE_TOCMODEL_H
