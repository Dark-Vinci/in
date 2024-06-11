// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "timetz", schema = "pg_catalog"))]
    pub struct Timetz;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Timetz;

    accounts (account_id) {
        account_id -> Int4,
        #[max_length = 50]
        password -> Nullable<Varchar>,
        #[max_length = 100]
        username -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        add_date -> Nullable<Timestamptz>,
        update_date -> Nullable<Timetz>,
    }
}

diesel::table! {
    actors (actor_id) {
        actor_id -> Int4,
        #[max_length = 150]
        first_name -> Nullable<Varchar>,
        #[max_length = 150]
        last_name -> Varchar,
        #[max_length = 1]
        gender -> Nullable<Bpchar>,
        date_of_birth -> Nullable<Date>,
        add_date -> Nullable<Date>,
        updated_date -> Nullable<Date>,
    }
}

diesel::table! {
    customers (customer_id) {
        customer_id -> Int4,
        #[max_length = 50]
        first_name -> Nullable<Varchar>,
        #[max_length = 50]
        last_name -> Nullable<Varchar>,
        #[max_length = 150]
        email -> Nullable<Varchar>,
        age -> Nullable<Int4>,
    }
}

diesel::table! {
    directors (director_id) {
        director_id -> Int4,
        #[max_length = 150]
        first_name -> Nullable<Varchar>,
        #[max_length = 150]
        last_name -> Nullable<Varchar>,
        date_of_birth -> Nullable<Date>,
        #[max_length = 20]
        nationality -> Nullable<Varchar>,
        add_date -> Nullable<Date>,
        update_date -> Nullable<Date>,
    }
}

diesel::table! {
    movies (movie_id) {
        movie_id -> Int4,
        #[max_length = 100]
        movie_name -> Varchar,
        movie_length -> Nullable<Int4>,
        #[max_length = 20]
        movie_lang -> Nullable<Varchar>,
        #[max_length = 10]
        age_certificate -> Nullable<Varchar>,
        release_date -> Nullable<Date>,
        director_id -> Nullable<Int4>,
    }
}

diesel::table! {
    movies_actors (movie_id, actor_id) {
        movie_id -> Int4,
        actor_id -> Int4,
    }
}

diesel::table! {
    movies_revenues (revenue_id) {
        movie_id -> Nullable<Int4>,
        revenues_domestic -> Nullable<Numeric>,
        revenues_international -> Nullable<Numeric>,
        revenue_id -> Float8,
    }
}

diesel::joinable!(movies -> directors (director_id));
diesel::joinable!(movies_actors -> actors (actor_id));
diesel::joinable!(movies_actors -> movies (movie_id));
diesel::joinable!(movies_revenues -> movies (movie_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    actors,
    customers,
    directors,
    movies,
    movies_actors,
    movies_revenues,
);
