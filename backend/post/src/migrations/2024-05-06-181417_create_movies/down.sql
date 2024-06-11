-- This file should undo anything in `up.sql`
CREATE TABLE "accounts"(
	"account_id" INT4 NOT NULL PRIMARY KEY,
	"password" VARCHAR(50),
	"username" VARCHAR(100),
	"email" VARCHAR(255),
	"add_date" TIMESTAMPTZ,
	"update_date" TIMETZ
);

CREATE TABLE "actors"(
	"actor_id" INT4 NOT NULL PRIMARY KEY,
	"first_name" VARCHAR(150),
	"last_name" VARCHAR(150) NOT NULL,
	"gender" BPCHAR(1),
	"date_of_birth" DATE,
	"add_date" DATE,
	"updated_date" DATE
);

CREATE TABLE "customers"(
	"customer_id" INT4 NOT NULL PRIMARY KEY,
	"first_name" VARCHAR(50),
	"last_name" VARCHAR(50),
	"email" VARCHAR(150),
	"age" INT4
);

CREATE TABLE "directors"(
	"director_id" INT4 NOT NULL PRIMARY KEY,
	"first_name" VARCHAR(150),
	"last_name" VARCHAR(150),
	"date_of_birth" DATE,
	"nationality" VARCHAR(20),
	"add_date" DATE,
	"update_date" DATE
);

CREATE TABLE "movies"(
	"movie_id" INT4 NOT NULL PRIMARY KEY,
	"movie_name" VARCHAR(100) NOT NULL,
	"movie_length" INT4,
	"movie_lang" VARCHAR(20),
	"age_certificate" VARCHAR(10),
	"release_date" DATE,
	"director_id" INT4,
	FOREIGN KEY ("director_id") REFERENCES "directors"("director_id")
);

CREATE TABLE "movies_actors"(
	"movie_id" INT4 NOT NULL,
	"actor_id" INT4 NOT NULL,
	PRIMARY KEY("movie_id", "actor_id"),
	FOREIGN KEY ("movie_id") REFERENCES "movies"("movie_id"),
	FOREIGN KEY ("actor_id") REFERENCES "actors"("actor_id")
);

CREATE TABLE "movies_revenues"(
	"movie_id" INT4,
	"revenues_domestic" NUMERIC,
	"revenues_international" NUMERIC,
	"revenue_id" FLOAT8 NOT NULL PRIMARY KEY,
	FOREIGN KEY ("movie_id") REFERENCES "movies"("movie_id")
);

DROP TABLE IF EXISTS "posts";
