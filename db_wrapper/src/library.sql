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

--name: insert_book!
--# Parameters
--param: uuid: &str			--param: path: &str
--param: duration: &str		--param: position: &str
--param: dir_uuid: &str		--param: navigation: &str
--param: title: &str       	--param: desc: &str
INSERT INTO book
( uuid, path, duration, position, dir_uuid, navigation,
  title, desc, identifiers)
VALUES
(:uuid,:path,:duration,:position,:dir_uuid,:navigation,
 :title,:desc,:identifiers);

-- name: get_books?
-- # Parameters
-- param: dir_uuid: &str
SELECT * FROM book WHERE dir_uuid = :dir_uuid;

-- name: get_creators?
-- # Parameters
-- param: book_uuid: &str
SELECT creator.uuid, creator.name, book_creator.type FROM creator
JOIN book_creator 	ON creator.uuid = book_creator.creator_uuid
JOIN book ON book_creator.book_uuid = book.uuid
WHERE book.uuid = :book_uuid;



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