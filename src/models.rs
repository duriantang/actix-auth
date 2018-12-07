use actix::{Actor, SyncContext};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbExecutor(pub PgPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

