use crate::ports::{
  element_repository_command_port::ElementRepositoryCommandPort,
  element_repository_query_port::ElementRepositoryQueryPort,
};

pub struct AppContext {
  query_repository: Box<dyn ElementRepositoryQueryPort>,
  command_repository: Box<dyn ElementRepositoryCommandPort>,
}

impl AppContext {
  pub fn new(
    query_repository: Box<dyn ElementRepositoryQueryPort>,
    command_repository: Box<dyn ElementRepositoryCommandPort>,
  ) -> Self {
    Self {
      query_repository,
      command_repository,
    }
  }

  pub fn get_query_repository(&self) -> &Box<dyn ElementRepositoryQueryPort> {
    &self.query_repository
  }

  pub fn get_command_repository(&self) -> &Box<dyn ElementRepositoryCommandPort> {
    &self.command_repository
  }
}
