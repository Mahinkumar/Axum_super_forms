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
     uqid   SERIAL PRIMARY KEY,
     userid INTEGER REFERENCES forms_user(userid),
     gid    INTEGER NOT NULL DEFAULT 1
  );

CREATE TABLE IF NOT EXISTS form_register
  (
     fid       SERIAL PRIMARY KEY,
     gid       INTEGER DEFAULT 1,
     form_name TEXT,
     form_description TEXT DEFAULT 'No Description Provided',
     startdate DATE DEFAULT CURRENT_DATE,
     starttime TIME DEFAULT CURRENT_TIME,
     enddate DATE DEFAULT NULL,
     endtime TIME DEFAULT NULL,
     dynamic BOOLEAN DEFAULT false
  );

CREATE TABLE IF NOT EXISTS forms
  (
     elid       SERIAL PRIMARY KEY,
     fid        SERIAL NOT NULL REFERENCES form_register(fid),
     typ        TEXT NOT NULL,
     req        BOOLEAN DEFAULT false,
     field_name VARCHAR NOT NULL,
     question   TEXT NOT NULL,
     limited    BOOLEAN DEFAULT false,
     limit_val  INTEGER DEFAULT 0
  );  

CREATE TABLE IF NOT EXISTS form_data
   (
      eid     SERIAL PRIMARY KEY,
      username    TEXT NOT NULL,
      user_id     INTEGER NOT NULL REFERENCES forms_user(userid),
      fid       SERIAL NOT NULL REFERENCES form_register(fid),
      input_name  TEXT NOT NULL,
      input_value TEXT NOT NULL
   );
