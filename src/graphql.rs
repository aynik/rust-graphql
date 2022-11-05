use crate::context::Context;
use crate::mutations::*;
use crate::queries::*;
use juniper::*;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
