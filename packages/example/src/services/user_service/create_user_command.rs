use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use kti_cqrs_rs::common::handler::CommandHandler;

use super::user_service::{User, UserService};

pub struct CreateUserCommand {
  name: String,
  email: String,
}

impl CreateUserCommand {
  pub fn new(name: &str, email: &str) -> Self {
    Self {
      name: name.to_string(),
      email: email.to_string(),
    }
  }
}

#[async_trait]
impl CommandHandler for CreateUserCommand {
  type Context = UserService;
  type Output = ();

  async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
    let mut user_service = context.lock().unwrap();

    user_service
      .create_user(User::new(&self.name, &self.email))
      .unwrap()
  }
}
