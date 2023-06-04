-- name: insert_dir!
-- # Parameters
-- param: dir_uuid: &str
-- param: name: &str
-- param: path: &str
-- param: parent_uuid: &str
INSERT INTO dir (dir_uuid, path, name , parent_uuid)
VALUES (:dir_uuid, :path, :name, :parent_uuid);

-- name: get_dirs?
-- # Parameters
-- param: dir_uuid: &str
SELECT dir_uuid, name FROM dir WHERE dir_uuid = :dir_uuid;

-- name: insert_media!
-- # Parameters
-- param: uuid: &str
-- param: path: &str
-- param: duration: &str
-- param: position: &str
-- param: dir_uuid: &str
-- param: navigation: &str
-- param: title: &str
-- param: desc: &str
INSERT INTO book
(uuid, path, duration, position, dir_uuid, navigation, title, desc)
VALUES
(:uuid, :path, :duration, :position, :dir_uuid, :navigation, :title, :desc);

-- name: get_book_creators?
-- # Parameters
-- param: book_uuid: &str
SELECT creator.uuid, creator.name, book_creator.type FROM creator
JOIN book_creator ON creator.uuid = book_creator.creator_uuid
JOIN book ON book_creator.book_uuid = book.uuid
WHERE book.uuid = :book_uuid;



-- name: get_book_subjects?
-- # Parameters
-- param: book_uuid: &str
SELECT subject.uuid, subject.name FROM subject
JOIN book_subject ON subject.uuid = book_subject.subject_uuid
JOIN book ON book_subject.book_uuid = book.uuid
WHERE book.uuid = :book_uuid;

-- name: get_media?
-- # Parameters
-- param: dir_uuid: &str
SELECT * FROM book WHERE dir_uuid = :dir_uuid;

-- name: set_media_position!
-- # Parameters
-- param: uuid: &str
-- param: position: &str
UPDATE book SET position = :position WHERE uuid = :uuid;

--name: select_media_position?
-- # Parameters
-- param: uuid: &str
SELECT position FROM book WHERE uuid = :uuid;