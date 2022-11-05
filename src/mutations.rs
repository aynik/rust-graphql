use crate::collectors::{UserCollector, UserFilter};
use crate::context::Context;
use crate::models::{UserCollection, UserInput};
use crate::mutators::UserMutator;
pub use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    async fn users(ctx: &Context, inputs: Vec<UserInput>) -> FieldResult<UserCollection> {
        use crate::schema::users::dsl::*;
        let conn = &mut ctx.pool.get().unwrap();
        let upsert_ids = UserMutator::upsert(conn, inputs);
        let upserted_users = UserCollector::new(users.into_boxed())
            .filter(UserFilter::new(Some(upsert_ids), None, None, None))
            .load(conn);
        Ok(UserCollection {
            count: upserted_users.len() as i32,
            items: upserted_users,
        })
    }
}
