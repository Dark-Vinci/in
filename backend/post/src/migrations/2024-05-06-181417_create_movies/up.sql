-- Your SQL goes here
DROP TABLE IF EXISTS "accounts";
DROP TABLE IF EXISTS "actors";
DROP TABLE IF EXISTS "customers";
DROP TABLE IF EXISTS "directors";
DROP TABLE IF EXISTS "movies";
DROP TABLE IF EXISTS "movies_actors";
DROP TABLE IF EXISTS "movies_revenues";
CREATE TABLE "posts"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"title" VARCHAR NOT NULL,
	"body" TEXT NOT NULL,
	"published" BOOL NOT NULL,
	"is_alive" BOOL NOT NULL,
	"is_man" BOOL NOT NULL
);

