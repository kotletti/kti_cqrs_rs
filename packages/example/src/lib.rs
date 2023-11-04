use std::{
  error::Error,
  sync::{Arc, Mutex},
};

use kti_cqrs_rs::core::bus::{CommandBus, QueryBus};
use services::user_service::{
  create_user_command::CreateUserCommand,
  get_user_by_name_query::GetUserByNameQuery,
  user_service::{User, UserService},
};

pub mod services;

pub struct TestSuit {
  service: Arc<Mutex<UserService>>,
}

impl TestSuit {
  pub fn new() -> Self {
    let users = [
      User::new("Andrey", "andrey@mail.domain"),
      User::new("Daria", "daria@mail.domain"),
      User::new("Kirill", "kirill@mail.domain"),
    ]
    .to_vec();

    let service = Arc::new(Mutex::new(UserService::new(users)));

    Self { service }
  }

  pub async fn get_user_by_name(&self, name: &str) -> Option<User> {
    let query_bus = QueryBus;

    let query = GetUserByNameQuery::new(name);

    let user = query_bus
      .send(Box::new(query), self.service.clone())
      .await
      .unwrap();

    user
  }

  pub async fn create_user(&self, name: &str, email: &str) -> Result<(), Box<dyn Error>> {
    let command_bus = CommandBus;

    let command = CreateUserCommand::new(&name, &email);

    command_bus
      .send(Box::new(command), self.service.clone())
      .await;

    Ok(())
  }
}

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn should_get_user_by_name() {
    let suit = TestSuit::new();

    let user_name = "Andrey";

    let user = suit.get_user_by_name(&user_name).await.unwrap();

    assert_eq!(user.get_name(), user_name);
  }

  #[tokio::test]
  async fn should_create_new_user() {
    let suit = TestSuit::new();

    let user_name = "Rita";

    let user_email = "rita@mail.domain";

    suit.create_user(&user_name, &user_email).await.unwrap();

    let user = suit.get_user_by_name(&user_name).await.unwrap();

    assert_eq!(user.get_name(), user_name);
    assert_eq!(user.get_email(), user_email);
  }
}
