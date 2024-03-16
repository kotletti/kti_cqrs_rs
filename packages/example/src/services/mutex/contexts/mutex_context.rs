use crate::services::mutex::ports::mutex_repository_port::{
  MutexRepositoryCommandPort, MutexRepositoryQueryPort,
};

pub struct MutexContext {
  query_repository: Box<dyn MutexRepositoryQueryPort>,
  command_repository: Box<dyn MutexRepositoryCommandPort>,
}

impl MutexContext {
  pub fn new(
    query_repository: Box<dyn MutexRepositoryQueryPort>,
    command_repository: Box<dyn MutexRepositoryCommandPort>,
  ) -> Self {
    Self {
      query_repository,
      command_repository,
    }
  }

  pub fn get_query_repository(&self) -> &Box<dyn MutexRepositoryQueryPort> {
    &self.query_repository
  }

  pub fn get_command_repository(&self) -> &Box<dyn MutexRepositoryCommandPort> {
    &self.command_repository
  }
}
