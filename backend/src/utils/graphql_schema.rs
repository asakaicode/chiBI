use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub struct Query;

#[Object]
impl Query {
  async fn hello(&self) -> String {
    "Hello, world".to_string()
  }
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .finish()
}