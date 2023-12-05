#include "librarymodel.h"
#include "rust_util.h"

LibraryModel::LibraryModel(QObject* parent) : QAbstractListModel(parent) {
}

void LibraryModel::openLibrary(const QString& uuid, const QString& path) {
	this->library_path = asRustStr(path);
	this->library_uuid = asRustStr(uuid);
	dir_stack.push( Dir {.uuid = "root", .name = "/" });
	open_library(this->library_uuid, this->library_path);
	updateMediaFiles();
}

QVariant LibraryModel::data(const QModelIndex& index, int role) const {
	int row = index.row();
	if		(row >= dirs.size()) return bookData(row - dirs.size(), role);
	else	return dirData(row, role);
}

QVariant LibraryModel::bookData(int row, int role) const {
	const auto [uuid, title, progress, cover_path] = books.at(row);
	switch (role) {
		case UUID:			return asQStr(uuid);
		case Name:			return asQStr(title);
		case Path:			return asQStr(get_book_path(library_uuid, uuid));
		case IsContainer:	return false;
		case Progress:		return progress;
		case HasCover:		return !cover_path.empty();
		case Cover:			return asQStr(cover_path);
		default:			return "";
	}
}

QVariant LibraryModel::dirData(int row, int role) const {
	const auto [uuid, name, cover_path] = dirs.at(row);
	switch (role) {
		case UUID:			return asQStr(uuid);
		case Name:			return asQStr(name);
		case IsContainer:	return true;
		case Path:			return "dir";
		case Progress:		return 0;
		case HasCover:		return !cover_path.empty();
		case Cover:			return asQStr(cover_path);
		default:			return "";
	}
}

int LibraryModel::rowCount(const QModelIndex& parent) const {
	return dirs.size() + books.size();
}

int LibraryModel::columnCount(const QModelIndex& parent) const { return 1; }

QHash<int, QByteArray> LibraryModel::roleNames() const {
	return {
		{UUID,			"uuid"},
		{Name,			"name"},
		{Progress,		"progress"},
		{Path,			"path"},
		{IsContainer,	"isContainer"},
		{HasCover,		"hasCover"},
		{Cover,			"cover"}
	};
}

void LibraryModel::scanLibrary() {
	scan_library(this->library_uuid, this->library_path);
	updateMediaFiles();
}

void LibraryModel::updateMediaFiles() {
	beginResetModel();
	this->dirs	= get_dirs(library_uuid, dir_stack.top().uuid);
	this->books = get_media_files(library_uuid, dir_stack.top().uuid);
	endResetModel();
}

void LibraryModel::setMediaPosition(const QString& uuid,
                                    const QString& location, int progress) {
	set_media_position(library_uuid, asRustStr(uuid), asRustStr(location),
	                   progress);
}

QString LibraryModel::getMediaPosition(const QString& uuid) {
	return asQStr(get_media_position(this->library_uuid, asRustStr(uuid)));
}

void LibraryModel::enterDir(int index) {
	dir_stack.push(dirs.at(index));
	updateMediaFiles();
}

bool LibraryModel::prevDir() {
	if (dir_stack.size() == 1) return false; // can't go back from root
	dir_stack.pop();
	updateMediaFiles();
	return true;
}

QString LibraryModel::getLibraryUuid() {
	return asQStr(this->library_uuid);
}

QString LibraryModel::deleteBook(const QString& uuid) {
	QString result = asQStr(delete_book(this->library_uuid, asRustStr(uuid)));
	updateMediaFiles();
	return result;
}

QString LibraryModel::deleteDir(const QString& uuid) {
	QString result = asQStr(delete_dir(this->library_uuid, asRustStr(uuid)));
	updateMediaFiles();
	return result;
}
