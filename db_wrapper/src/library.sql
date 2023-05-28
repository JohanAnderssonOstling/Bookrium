-- name: insert_dir!
-- # Parameters
-- param: dir_uuid: &str
-- param: name: &str
-- param: path: &str
-- param: parent_dir_uuid: &str
INSERT INTO dir (dir_uuid, path, name , parent_dir_uuid)
VALUES (:dir_uuid, :path, :name, :parent_dir_uuid);

-- name: get_dirs?
-- # Parameters
-- param: parent_dir_uuid: &str
SELECT dir_uuid, name FROM dir WHERE parent_dir_uuid = :parent_dir_uuid;

-- name: insert_media!
-- # Parameters
-- param: media_uuid: &str
-- param: path: &str
-- param: duration: &str
-- param: position: &str
-- param: parent_dir_uuid: &str
INSERT INTO media (media_uuid, path, duration, position, parent_dir_uuid)
VALUES (:media_uuid, :path, :duration, :position, :parent_dir_uuid);


-- name: get_media?
-- # Parameters
-- param: parent_dir_uuid: &str
SELECT * FROM media WHERE parent_dir_uuid = :parent_dir_uuid;

-- name: set_media_position!
-- # Parameters
-- param: media_uuid: &str
-- param: position: &str
UPDATE media SET position = :position WHERE media_uuid = :media_uuid;

--name: select_media_position?
-- # Parameters
-- param: media_uuid: &str
SELECT position FROM media WHERE media_uuid = :media_uuid;