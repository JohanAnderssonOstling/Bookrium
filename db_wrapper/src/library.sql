-- BOOKS -------------------------------------------------------
-- name: select_book_uuid?
-- # Parameters
-- param: file_name: &str
SELECT uuid FROM book WHERE file_name = :file_name;

-- name: select_book?
-- # Parameters
-- param: uuid: &str
SELECT uuid FROM book WHERE uuid = :uuid;

--name: 	insert_book!
--# Parameters
--param: 	uuid: 		&str
--param: 	file_name: 	&str
--param: 	progress: 	u8
--param: 	position: 	&str
--param: 	navigation: 	&str
--param: 	title: 		&str
--param: 	desc: 		&str
--param: 	identifiers: 	&str
--param: 	published: 	u32
--param: 	scan_timestamp: u64
INSERT INTO book
  ( uuid, file_name, progress, position, navigation,
    title, desc, identifiers, published, library_add_timestamp)
VALUES
  (:uuid,:file_name,:progress,:position,:navigation,
   :title,:desc,:identifiers,:published,:scan_timestamp);

--name: 	delete_book!
--# Parameters
--param: 	uuid: &str
DELETE FROM book WHERE uuid = :uuid;

-- name: 	get_books?
--#Parameters
-- param: 	dir_uuid: 	&str
SELECT book.uuid, book.title, book.progress FROM book
JOIN book_dir ON book.uuid = book_dir.book_uuid
JOIN dir ON book_dir.dir_uuid = dir.dir_uuid
WHERE dir.dir_uuid = :dir_uuid;

-- name: get_book_file_info?
-- # Parameters
-- param: book_uuid: &str
SELECT book.file_name, book_dir.dir_uuid
FROM book
JOIN book_dir ON book.uuid = book_dir.book_uuid
WHERE book.uuid = :book_uuid LIMIT 1;

-- name: get_book_toc?
-- param: book_uuid: &str
SELECT navigation FROM book WHERE uuid = :book_uuid;


-- DIRS --------------------------------------------------------

-- name: insert_dir!
-- param: dir_uuid: 	&str
-- param: name: 	&str
-- param: parent_uuid: 	&str
INSERT INTO dir ( dir_uuid, dir_name, parent_uuid)
VALUES 		(:dir_uuid,:name,:parent_uuid);

--name: 	delete_dir!
--# Parameters
--param: 	dir_uuid: &str
DELETE FROM dir WHERE dir_uuid = :dir_uuid;

-- name: get_dirs?
-- # Parameters
-- param: dir_uuid: &str
SELECT dir_uuid, dir_name, parent_uuid
FROM dir WHERE parent_uuid = :dir_uuid;

-- name: select_dir?
-- # Parameters
-- param: dir_uuid: &str
SELECT * FROM dir WHERE dir_uuid = :dir_uuid;

-- name: insert_book_dir!
-- # Parameters
-- param: book_uuid: &str
-- param: dir_uuid: &str
INSERT INTO book_dir 	( book_uuid, dir_uuid)
VALUES 			(:book_uuid,:dir_uuid);

-- name: set_pos!
-- # Parameters
-- param: uuid: &str
-- param: position: &str
-- param: progress: u8
UPDATE book SET position = :position, progress = :progress WHERE uuid = :uuid;

--name: get_pos?
-- # Parameters
-- param: uuid: &str
SELECT position FROM book WHERE uuid = :uuid;