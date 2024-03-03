use crate::services::complex::ports::complex_repository_port::{
  ComplexRepositoryCommandPort, ComplexRepositoryQueryPort,
};

pub struct ComplexContext {
  query_repository: Box<dyn ComplexRepositoryQueryPort>,
  command_repository: Box<dyn ComplexRepositoryCommandPort>,
}

impl ComplexContext {
  pub fn new(
    query_repository: Box<dyn ComplexRepositoryQueryPort>,
    command_repository: Box<dyn ComplexRepositoryCommandPort>,
  ) -> Self {
    Self {
      query_repository,
      command_repository,
    }
  }

  pub fn get_query_repository(&self) -> &Box<dyn ComplexRepositoryQueryPort> {
    &self.query_repository
  }

  pub fn get_command_repository(&self) -> &Box<dyn ComplexRepositoryCommandPort> {
    &self.command_repository
  }
}
