use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};

use crate::db::Pool;
pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}
pub struct QueryRoot;
#[juniper::graphql_object(Context = Context)]
impl QueryRoot {

    fn hello() -> FieldResult<String> {
        Ok("Hello, world!".to_string())
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context) -> FieldResult<i32> {
        println!("test");
        Ok(2)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}