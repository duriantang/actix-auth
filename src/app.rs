use crate::models::DbExecutor;
use ::actix::prelude::*;
use actix_web::{http::Method, middleware, App};

type AddrDbExecutor = Addr<DbExecutor>;

pub struct AppState {
    pub db: AddrDbExecutor,
}

pub fn create_app(db: AddrDbExecutor) -> App<AppState> {
    App::with_state(AppState { db })
        .middleware(middleware::Logger::new("\"%r\" %s %b %Dms"))
        .resource("/auth", |r| {})
        .resource("/invitation/", |r| {})
        .resource("/register/", |r| {})
}
