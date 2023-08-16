#include "librarymodel.h"
#include "rust_util.h"

LibraryModel::LibraryModel(QObject *parent) : QAbstractListModel(parent) {}

void LibraryModel::openLibrary(const QString &uuid, const QString &path) {
  this->library_path = asRustStr(path);
  this->library_uuid = asRustStr(uuid);
  dir_stack.push( Dir {.uuid = "root", .name = "/"} );
  open_library(this->library_uuid, this->library_path);
  updateMediaFiles();
}


QVariant LibraryModel::data(const QModelIndex &index, int role) const {
  qInfo() << "data" << index.row() << role;
  int row = index.row();
  if (row >= dirs.size()) return bookData(row - dirs.size(), role);
  else return dirData(row, role);
}

QVariant LibraryModel::bookData(int row, int role) const {
  const CXXBook book = books.at(row);
  switch (role) {
    case UUID: return asQStr (book.uuid);
    case Name: return asQStr (book.title);
    case Path: return asQStr (get_book_path(library_uuid, book.uuid));
	case IsContainer: return false;
    case HasCover: return !book.cover_path.empty();
    case Cover: return asQStr (book.cover_path);
  }
}

QVariant LibraryModel::dirData(int row, int role) const {
  const Dir dir = dirs.at(row);
  switch (role) {
    case UUID: return asQStr (dir.uuid);
    case Name: return asQStr (dir.name);
	case IsContainer: return true;
    case Path: return "dir";
    case HasCover: return !dir.cover_path.empty();
    case Cover: return asQStr (dir.cover_path);
  }
}
int LibraryModel::rowCount(const QModelIndex &parent) const {
  return dirs.size() + books.size();
}

int LibraryModel::columnCount(const QModelIndex &parent) const { return 1; }

QHash<int, QByteArray> LibraryModel::roleNames() const {
  return {{UUID,     "uuid"},
          {Name,     "name"},
          {Path,     "path"},
		  {IsContainer, "isContainer"},
          {HasCover, "hasCover"},
          {Cover,    "cover"}};
}

void LibraryModel::scanLibrary() {
  scan_library(this->library_uuid, this->library_path);
  updateMediaFiles();
}

void LibraryModel::updateMediaFiles() {
  beginResetModel();
  this->dirs = get_dirs(library_uuid, dir_stack.top().uuid);
  this->books = get_media_files(library_uuid, dir_stack.top().uuid);
  endResetModel();
}

void LibraryModel::setMediaPosition(const QString &uuid, const QString &location) {
  set_media_position(library_uuid, asRustStr(uuid), asRustStr(location));
}

QString LibraryModel::getMediaPosition(const QString &uuid) {
  return asQStr(get_media_position(this->library_uuid, asRustStr(uuid)));
}

void LibraryModel::enterDir(int index) {
  dir_stack.push(dirs.at(index));
  updateMediaFiles();
}

bool LibraryModel::prevDir() {
  if (dir_stack.size() == 1) return false;                      // can't go back from root
  dir_stack.pop();
  updateMediaFiles();
  return true;
}

QString LibraryModel::getLibraryUuid() {
	return asQStr(this->library_uuid);
}
