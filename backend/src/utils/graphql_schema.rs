use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

use crate::utils::graphql_context::GraphQLContext;

pub struct Query;

#[Object]
impl Query {
  async fn hello(&self, ctx: &Context<'_>) -> String {
    let my_context = ctx.data::<GraphQLContext>().unwrap();
    format!("Hello, world, {}", my_context.user_id)
  }
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .finish()
}