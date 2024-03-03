use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_rs::common::handler::QueryHandler;
use tokio::sync::Mutex;

use super::user_service::{User, UserService};

pub struct GetUserByNameQuery {
  name: String,
}

impl GetUserByNameQuery {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
    }
  }
}

#[async_trait]
impl QueryHandler for GetUserByNameQuery {
  type Context = UserService;
  type Output = Result<Option<User>, Box<dyn Error>>;

  async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
    let user_service = context.lock().await;

    user_service.get_user_by_name(&self.name)
  }
}
