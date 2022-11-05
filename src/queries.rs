use crate::collectors::{Pagination, UserCollector, UserFilter};
use crate::context::Context;
use crate::models::UserCollection;
use crate::schema::users;
pub use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    fn users(
        ctx: &Context,
        ids: Option<Vec<Uuid>>,
        usernames: Option<Vec<String>>,
        names: Option<Vec<String>>,
        friend_ids: Option<Vec<Uuid>>,
        after: Option<Uuid>,
        limit: Option<i32>,
        reversed: Option<bool>,
    ) -> FieldResult<UserCollection> {
        let conn = &mut ctx.pool.get().unwrap();
        let user_filter = UserFilter::new(ids, usernames, names, friend_ids);
        Ok(UserCollection {
            count: UserCollector::new(users::table.into_boxed())
                .filter(user_filter.clone())
                .count(conn),
            items: UserCollector::new(users::table.into_boxed())
                .filter(user_filter.clone())
                .paginate(Pagination::new(after, limit, reversed))
                .load(conn),
        })
    }
}
