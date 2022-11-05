use crate::context::Context;
use crate::schema::users;
use diesel::prelude::*;
use juniper::{graphql_object, GraphQLInputObject};
use uuid::Uuid;

#[derive(Queryable, Insertable, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub friend_ids: Vec<Uuid>,
}

#[derive(GraphQLInputObject)]
pub struct UserInput {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub friend_ids: Vec<Uuid>,
}

impl UserInput {
    pub fn into_user(self) -> User {
        User {
            id: self.id,
            username: self.username,
            name: self.name,
            friend_ids: self.friend_ids,
        }
    }
}

#[graphql_object(Context = Context)]
impl User {
    fn id(&self) -> Uuid {
        self.id
    }

    fn username(&self) -> String {
        self.username.to_string()
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    async fn friends(&self, ctx: &Context) -> UserCollection {
        let loaded_friends = ctx.user_loader.load_many(self.friend_ids.clone()).await;
        UserCollection {
            count: self.friend_ids.len() as i32,
            items: self
                .friend_ids
                .iter()
                .map(|id| loaded_friends[&id].clone())
                .collect(),
        }
    }
}

pub struct UserCollection {
    pub count: i32,
    pub items: Vec<User>,
}

#[graphql_object(Context = Context)]
impl UserCollection {
    fn count(&self) -> i32 {
        self.count
    }

    fn items(&self) -> Vec<User> {
        self.items.clone()
    }
}
