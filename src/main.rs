#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

mod app;
mod errors;
mod invitation_handler;
mod invitation_routes;
mod models;
mod schema;

use crate::models::DbExecutor;
use ::actix::prelude::*;
use actix_web::server;
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    info!("enviroment init success");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sys = actix::System::new("Actix_Tutorial");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let address: Addr<DbExecutor> = SyncArbiter::start(4, move || {
        info!("start pool");
        DbExecutor(pool.clone())
    });

    server::new(move || {
        info!("create app");
        app::create_app(address.clone())
    })
    .bind("127.0.0.1:3000")
    .expect("Can not bind to '127.0.0.1:3000")
    .start();
    info!("start sys");
    sys.run();
}
