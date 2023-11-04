use std::error::Error;

#[derive(Clone)]
pub struct User {
  name: String,
  email: String,
}

impl User {
  pub fn new(name: &str, email: &str) -> Self {
    Self {
      name: name.to_string(),
      email: email.to_string(),
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_email(&self) -> &str {
    &self.email
  }
}

#[derive(Clone)]
pub struct UserService {
  users: Vec<User>,
}

impl UserService {
  pub fn new(users: Vec<User>) -> Self {
    Self { users }
  }

  pub fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
    Ok(self.users.clone())
  }

  pub fn get_user_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn Error>> {
    Ok(self.users.iter().find(|i| i.name.eq(name)).cloned())
  }

  pub fn create_user(&mut self, user: User) -> Result<(), Box<dyn Error>> {
    self.users.push(user);

    Ok(())
  }

  pub fn update_user_email(&mut self, name: &str, email: &str) -> Result<(), Box<dyn Error>> {
    let index = match self.users.iter().position(|i| i.name == name) {
      Some(r) => r,
      None => return Err("Cant find user by name.".into()),
    };

    self.users.remove(index);

    self.users.push(User::new(name, email));

    Ok(())
  }
}
