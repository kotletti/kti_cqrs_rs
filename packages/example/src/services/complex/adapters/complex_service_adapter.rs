use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_rs::core::bus::{CommandBus, QueryBus};
use tokio::sync::Mutex;

use crate::services::complex::{
  commands::{add_element_command, remove_element_command},
  contexts::complex_context::ComplexContext,
  ports::complex_service_port::ComplexServicePort,
  queries::{get_count_elements_query, get_elements_query},
};

pub struct ComplexServiceAdapter {
  context: Arc<Mutex<ComplexContext>>,
  command: CommandBus,
  query: QueryBus,
}

impl ComplexServiceAdapter {
  pub fn new(context: Arc<Mutex<ComplexContext>>, command: CommandBus, query: QueryBus) -> Self {
    Self {
      context,
      command,
      query,
    }
  }
}

#[async_trait]
impl ComplexServicePort for ComplexServiceAdapter {
  async fn get_elements(&self) -> Result<Vec<i32>, Box<dyn Error>> {
    let query = get_elements_query::Query;

    let elements = self
      .query
      .send(Box::new(query), self.context.clone())
      .await?;

    Ok(elements)
  }

  async fn add_element(&self, element: i32) -> Result<(), Box<dyn Error>> {
    let command = add_element_command::Command { element };

    self
      .command
      .send(Box::new(command), self.context.clone())
      .await
  }

  async fn remove_element(&self, element: i32) -> Result<(), Box<dyn Error>> {
    let command = remove_element_command::Command { element };

    self
      .command
      .send(Box::new(command), self.context.clone())
      .await
  }

  async fn get_count(&self) -> Result<i32, Box<dyn Error>> {
    let query = get_count_elements_query::Query;

    let count = self
      .query
      .send(Box::new(query), self.context.clone())
      .await?;

    Ok(count)
  }
}
