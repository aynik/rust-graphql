use crate::models::User;
use crate::schema::users;
use derive_more::Constructor;
use diesel::pg::Pg;
pub use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{ExpressionMethods, PgArrayExpressionMethods, QueryDsl};
use uuid::Uuid;

#[derive(Constructor, Copy, Clone)]
pub struct Pagination {
    after: Option<Uuid>,
    limit: Option<i32>,
    reversed: Option<bool>,
}

#[derive(Constructor, Clone)]
pub struct UserFilter {
    ids: Option<Vec<Uuid>>,
    usernames: Option<Vec<String>>,
    names: Option<Vec<String>>,
    friend_ids: Option<Vec<Uuid>>,
}

#[derive(Constructor)]
pub struct UserCollector {
    pub query: users::BoxedQuery<'static, Pg>,
}

impl UserCollector {
    pub fn filter(mut self, filter: UserFilter) -> Self {
        use crate::schema::users::dsl::*;
        if let Some(ids) = filter.ids {
            self.query = self.query.filter(id.eq_any(ids));
        }
        if let Some(usernames) = filter.usernames {
            self.query = self.query.filter(username.eq_any(usernames));
        }
        if let Some(names) = filter.names {
            self.query = self.query.filter(name.eq_any(names));
        }
        if let Some(_friend_ids) = filter.friend_ids {
            self.query = self.query.filter(friend_ids.overlaps_with(_friend_ids));
        }
        return self;
    }

    pub fn paginate(mut self, pagination: Pagination) -> Self {
        use crate::schema::users::dsl::*;
        if let Some(after) = pagination.after {
            if pagination.reversed.unwrap_or(false) {
                self.query = self.query.filter(id.lt(after));
            } else {
                self.query = self.query.filter(id.gt(after));
            }
        }
        if let Some(limit) = pagination.limit {
            self.query = self.query.limit(limit.clone() as i64);
        }
        if pagination.reversed.unwrap_or(false) {
            self.query = self.query.order(id.desc());
        } else {
            self.query = self.query.order(id.asc());
        }
        return self;
    }

    pub fn load(self, conn: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Vec<User> {
        return self.query.load::<User>(conn).expect("Error loading users");
    }

    pub fn count(self, conn: &mut PooledConnection<ConnectionManager<PgConnection>>) -> i32 {
        return self.query.count().first::<i64>(conn).unwrap() as i32;
    }
}
