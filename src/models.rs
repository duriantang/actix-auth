use crate::schema::{invitations, users};
use actix::{Actor, SyncContext};
use chrono::{Local, NaiveDateTime};
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
    pub fn with_details(email: String, password: String) -> Self {
        User {
            email,
            password,
            created_at: Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { email: user.email }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: NaiveDateTime,
}
