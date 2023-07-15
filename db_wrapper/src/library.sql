-- DIRS
-- name: insert_dir!
-- # Parameters
-- param: dir_uuid: &str
-- param: path: &str
-- param: parent_uuid: &str
INSERT INTO dir (dir_uuid, path, parent_uuid)
VALUES (:dir_uuid, :path, :parent_uuid);

-- name: get_dirs?
-- # Parameters
-- param: dir_uuid: &str
SELECT dir_uuid FROM dir WHERE dir_uuid = :dir_uuid;

-- BOOKS
-- name: select_book_uuid?
-- # Parameters
-- param: file_name: &str
SELECT uuid FROM book WHERE file_name = :file_name;


--name: insert_book!
--# Parameters
--param: uuid: &str
--param: file_name: &str
--param: progress: u8
--param: position: &str
--param: dir_uuid: &str
--param: navigation: &str
--param: title: &str
--param: desc: &str
--param: identifiers: &str
--param: published: u32
INSERT INTO book
( uuid, file_name, progress, position, dir_uuid, navigation,
  title, desc, identifiers,published)
VALUES
(:uuid,:file_name,:progress,:position,:dir_uuid,:navigation,
 :title,:desc,:identifiers,:published);

-- name: get_books?
-- # Parameters
-- param: dir_uuid: &str
SELECT uuid, title, progress FROM book WHERE dir_uuid = :dir_uuid;

-- CREATORS
-- name: select_creator_uuid?
-- # Parameters
-- param: creator_name: &str
SELECT uuid FROM creator WHERE :creator_name = ?;

-- name: insert_creator!
-- # Parameters
-- param: uuid: &str
-- param: name: &str
INSERT INTO creator (uuid, name) VALUES (:uuid, :name);



-- name: get_creators?
-- # Parameters
-- param: book_uuid: &str
SELECT creator.uuid, creator.name, book_creator.type FROM creator
JOIN book_creator 	ON creator.uuid = book_creator.creator_uuid
JOIN book ON book_creator.book_uuid = book.uuid
WHERE book.uuid = :book_uuid;


-- SUBJECTS
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
-- param: uuid: &str	-- param: position: &str
UPDATE book SET position = :position WHERE uuid = :uuid;

--name: get_pos?
-- # Parameters
-- param: uuid: &str
SELECT position FROM book WHERE uuid = :uuid;