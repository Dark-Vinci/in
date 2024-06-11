use {
    diesel::{Identifiable, Insertable, Queryable},
    serde::{Deserialize, Serialize},
    crate::models::db::account::schema::posts,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Identifiable,
    Insertable,
    Queryable,
)]
#[diesel(table_name = posts, primary_key(account_id))]
pub struct User {
    pub account_id: i32,
}
