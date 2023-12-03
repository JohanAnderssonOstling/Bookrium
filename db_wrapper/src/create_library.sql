-- name: create_book_table!

create table if not exists book(
  uuid		TEXT primary key,
  name     TEXT    not null,
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

create table if not exists book_link(
  linker_uuid	TEXT,
  linkee_uuid	TEXT,
  PRIMARY KEY (linker_uuid, linkee_uuid),
  FOREIGN KEY (linker_uuid) REFERENCES book (uuid) ON DELETE CASCADE,
  FOREIGN KEY (linkee_uuid) REFERENCES book (uuid) ON DELETE CASCADE
);

-- name: create_dir_table!
create table if not exists dir (
  uuid    		TEXT primary key,
  parent_uuid 	TEXT,
  name    		TEXT not null
);

-- name: create_book_dir_table!
create table if not exists book_dir(
	book_uuid	TEXT,
	dir_uuid	TEXT,
	PRIMARY KEY (book_uuid, dir_uuid),
	FOREIGN KEY (book_uuid) REFERENCES book (uuid) ON DELETE CASCADE,
	FOREIGN KEY (dir_uuid)	REFERENCES dir	(uuid) ON DELETE CASCADE
);


-- name: create_container_table!
create table if not exists container(
  uuid	TEXT primary key,
  name	TEXT,
  desc	TEXT,
  type	INTEGER
);

-- name: create_book_container_table!
create table if not exists book_container(
	book_uuid		TEXT,
	container_uuid	TEXT,
	PRIMARY KEY (book_uuid, container_uuid),
	FOREIGN KEY (book_uuid) REFERENCES book (uuid) ON DELETE CASCADE,
	FOREIGN KEY (container_uuid) REFERENCES container (uuid) ON DELETE CASCADE
);

-- name: create_container_link_table!
create table if not exists container_link(
  linker_uuid	TEXT,
  linkee_uuid	TEXT,
  PRIMARY KEY (linker_uuid, linkee_uuid),
  FOREIGN KEY (linker_uuid) REFERENCES container (uuid) ON DELETE CASCADE,
  FOREIGN KEY (linkee_uuid) REFERENCES container (uuid) ON DELETE CASCADE
);

-- name: create_container_alias_table!
create table if not exists container_alias(
  name	TEXT primary key,
  uuid	TEXT,
	FOREIGN KEY (uuid) REFERENCES container (uuid) ON DELETE CASCADE
);