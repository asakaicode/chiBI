pub struct GraphQLContext {
  pub user_id: String,
}

impl GraphQLContext {
  pub fn new(user_id: String) -> Self {
    GraphQLContext { user_id }
  }
}