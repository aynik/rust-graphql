use crate::collectors::{UserCollector, UserFilter};
use crate::models::User;
use crate::schema::users;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, QueryDsl};
use juniper::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

pub struct UserBatcher {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait]
impl BatchFn<Uuid, User> for UserBatcher {
    async fn load(&mut self, keys: &[Uuid]) -> HashMap<Uuid, User> {
        let users_query = UserCollector::new(users::table.into_boxed())
            .filter(UserFilter::new(Some(keys.to_vec()), None, None, None))
            .load(&mut self.pool.get().unwrap());
        let mut user_hashmap = HashMap::new();
        for user in users_query {
            user_hashmap.insert(user.id, user);
        }
        user_hashmap
    }
}

pub fn get_user_loader(
    pool: Pool<ConnectionManager<PgConnection>>,
) -> Loader<Uuid, User, UserBatcher> {
    Loader::new(UserBatcher { pool }).with_yield_count(100)
}
