-- name: create_book_table!

create table if not exists book(
  uuid		TEXT primary key,
  file_name     TEXT    not null,
  progress      INTEGER not null,
  position      TEXT,
  navigation    TEXT,
  title         TEXT,
  desc          TEXT,
  identifiers   TEXT,
  published     INTEGER,
  last_read     INTEGER,
  library_add_timestamp INTEGER
);

-- name: create_creator_table!
create table if not exists creator(
  uuid        TEXT primary key,
  name        TEXT not null,
  description TEXT
);

-- name: create_book_creator_table!
create table if not exists book_creator(
  book_uuid    TEXT references book,
  creator_uuid TEXT references creator,
  type         TEXT not null
);

-- name: create_dir_table!
create table if not exists dir (
  dir_uuid    TEXT primary key,
  parent_uuid TEXT,
  dir_name    TEXT not null
);

-- name: create_book_dir_table!
create table if not exists book_dir (
  book_uuid TEXT references book on delete cascade,
  dir_uuid  TEXT references dir  on delete cascade,
  primary key (book_uuid, dir_uuid)
);

-- name: create_subject_table!
create table if not exists subject(
  uuid		TEXT primary key,
  parent_uuid	TEXT,
  name 		TEXT
);

-- name: create_book_subject_table!
create table if not exists book_subject(
  book_uuid    TEXT,
  subject_uuid TEXT
);

