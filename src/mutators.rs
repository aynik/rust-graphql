use crate::models::{User, UserInput};
pub use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::upsert::excluded;
use diesel::{PgConnection, RunQueryDsl};
use uuid::Uuid;

pub struct UserMutator;

impl UserMutator {
    pub fn upsert(
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        inputs: Vec<UserInput>,
    ) -> Vec<Uuid> {
        use crate::schema::users::dsl::*;
        let upsert_users: Vec<User> = inputs.into_iter().map(|input| input.into_user()).collect();
        diesel::insert_into(users)
            .values(&upsert_users)
            .on_conflict(id)
            .do_update()
            .set((
                username.eq(excluded(username)),
                name.eq(excluded(name)),
                friend_ids.eq(excluded(friend_ids)),
            ))
            .execute(conn)
            .expect("Error mutating users");
        return upsert_users.into_iter().map(|user| user.id).collect();
    }
}
