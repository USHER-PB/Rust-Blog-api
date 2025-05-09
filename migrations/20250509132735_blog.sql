-- Add migration script here
 CREATE TABLE blog(
   title TEXT NOT NULL,
   body TEXT NOT NULL,
   author_id  int,
   publisher TEXT NOT NULL
  );