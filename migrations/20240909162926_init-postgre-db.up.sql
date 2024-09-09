 CREATE TABLE IF NOT EXISTS admins
  (
     aid      INTEGER PRIMARY KEY,
     email    VARCHAR,
     username TEXT,
     passhash TEXT
  );

CREATE TABLE IF NOT EXISTS forms_user
  (
     userid   INTEGER PRIMARY KEY,
     email    VARCHAR,
     username TEXT,
     passkey  VARCHAR
  );

CREATE TABLE IF NOT EXISTS user_group
  (
     uqid   INTEGER PRIMARY KEY,
     userid INTEGER REFERENCES forms_user(userid),
     gid    INTEGER NOT NULL DEFAULT 1
  );

CREATE TABLE IF NOT EXISTS form_register
  (
     fid       TEXT PRIMARY KEY,
     gid       INTEGER DEFAULT 1,
     form_name TEXT
  );

CREATE TABLE IF NOT EXISTS forms
  (
     elid       INTEGER PRIMARY KEY,
     fid        TEXT NOT NULL REFERENCES form_register(fid),
     typ        TEXT NOT NULL,
     req        BOOLEAN DEFAULT false,
     field_name VARCHAR NOT NULL,
     question   TEXT NOT NULL,
     limited    BOOLEAN DEFAULT false,
     limit_val  INTEGER DEFAULT 0
  );  
