use crate::services::rw_lock::ports::rw_lock_repository_port::{
  RwLockRepositoryCommandPort, RwLockRepositoryQueryPort,
};

pub struct RwLockContext {
  query_repository: Box<dyn RwLockRepositoryQueryPort>,
  command_repository: Box<dyn RwLockRepositoryCommandPort>,
}

impl RwLockContext {
  pub fn new(
    query_repository: Box<dyn RwLockRepositoryQueryPort>,
    command_repository: Box<dyn RwLockRepositoryCommandPort>,
  ) -> Self {
    Self {
      query_repository,
      command_repository,
    }
  }

  pub fn get_query_repository(&self) -> &Box<dyn RwLockRepositoryQueryPort> {
    &self.query_repository
  }

  pub fn get_command_repository(&self) -> &Box<dyn RwLockRepositoryCommandPort> {
    &self.command_repository
  }
}
