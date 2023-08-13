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

-- ENTRIES -----------------------------------------------------

-- name: get_entry?
-- param: table_name: &str
-- param: entry_name: &str
SELECT uuid FROM :table_name WHERE name = :entry_name;

-- name: insert_entry!
-- param: table_name: &str
-- param: uuid: &str
-- param: name: &str
INSERT INTO :table_name (uuid, name) VALUES (:uuid, :name);

-- name: insert_book_entry!
-- param: table_name: &str
-- param: book_uuid
-- param: container_uuid
INSERT INTO :table_name	( book_uuid, container_uuid)
VALUES 			(:book_uuid,:container_uuid);

-- DIRS --------------------------------------------------------

-- name: insert_dir!
-- param: dir_uuid: 	&str
-- param: name: 	&str
-- param: parent_uuid: 	&str
INSERT INTO dir ( dir_uuid, dir_name, parent_uuid)
VALUES 		(:dir_uuid,:name,:parent_uuid);

-- name: get_dirs?
-- # Parameters
-- param: dir_uuid: &str
SELECT dir_uuid, dir_name, parent_uuid
FROM dir WHERE parent_uuid = :dir_uuid;

-- name: clear_dirs!
-- # Parameters
-- param: test
DELETE FROM dir;

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

-- CREATORS ----------------------------------------------------

-- name: select_creator_uuid?
-- param: creator_name: &str
SELECT uuid FROM creator WHERE :creator_name = ?;

-- name: insert_creator!
-- # Parameters
-- param: uuid: &str
-- param: name: &str
INSERT INTO creator (uuid, name) VALUES (:uuid, :name);

-- name: get_book_creators?
-- # Parameters
-- param: book_uuid: &str
SELECT creator.uuid, creator.name, book_creator.type FROM creator
JOIN book_creator ON creator.uuid = book_creator.creator_uuid
JOIN book ON book_creator.book_uuid = book.uuid
WHERE book.uuid = :book_uuid;


-- SUBJECTS ----------------------------------------------------
-- name: select_subject_uuid?
-- # Parameters
-- param: subject_name: &str
SELECT uuid FROM subject WHERE :subject_name = ?;

-- name: insert_subject!
-- # Parameters
-- param: uuid: &str
-- param: name: &str
INSERT INTO subject (uuid, name) VALUES (:uuid, :name);

-- name: get_subjects?
-- # Parameters
-- param: book_uuid: &str
SELECT subject.uuid, subject.name FROM subject
  JOIN book_subject ON subject.uuid = book_subject.subject_uuid
JOIN book ON book_subject.book_uuid = book.uuid
WHERE book.uuid = :book_uuid;


-- name: set_pos!
-- # Parameters
-- param: uuid: &str
-- param: position: &str
UPDATE book SET position = :position WHERE uuid = :uuid;

--name: get_pos?
-- # Parameters
-- param: uuid: &str
SELECT position FROM book WHERE uuid = :uuid;