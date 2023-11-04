# Implementation of CQRS pattern in Rust

### Currently the crate contains only query & command handlers

Simple example (existed in repo)

```
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
```
