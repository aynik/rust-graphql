use crate::loaders::UserBatcher;
use crate::models::*;
use dataloader::non_cached::Loader;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

pub struct Context {
    pub pool: Pool<ConnectionManager<PgConnection>>,
    pub user_loader: Loader<Uuid, User, UserBatcher>,
}

impl juniper::Context for Context {}
