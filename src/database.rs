use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> PostgresPool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let migr = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .build(migr)
        .expect("could not build connection pool")
}
