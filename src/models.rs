use crate::schema::{invitations, users};
use actix::{Actor, SyncContext};
use chrono::NaiveDateTime;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use uuid::Uuid;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbExecutor(pub PgPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn remove_pwd(mut self) -> Self {
        self.password = "".to_string();
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: NaiveDateTime,
}
