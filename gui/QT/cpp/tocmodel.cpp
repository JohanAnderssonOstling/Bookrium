//
// Created by johan on 2023-08-16.
//

#include "tocmodel.h"
#include "rust_util.h"
QVariant
TocModel::data(const QModelIndex &index, int role) const {
	Nav nav = this->toc.at(index.row());
	int row = index.row();
	switch (role) {
		case Name: return asQStr(nav.name);
		case Href: return asQStr(nav.href);
		default: return {};
	}
}

QHash<int, QByteArray> TocModel::roleNames() const {
	return { {Name, "name"}, {Href, "href"} };
}

void TocModel::setToc(QString library_uuid, QString book_uuid) {
	beginResetModel();
	this->toc = get_book_toc(asRustStr(library_uuid), asRustStr(book_uuid));
	endResetModel();
}
