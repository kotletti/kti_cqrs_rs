use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_rs::core::bus::{command_bus::CommandBus, query_bus::QueryBus};
use tokio::sync::Mutex;

use crate::services::mutex::{
  contexts::mutex_context::MutexContext, ports::mutex_service_port::MutexServicePort,
};

use crate::services::mutex::commands::{add_element_command, remove_element_command};
use crate::services::mutex::queries::{get_count_elements_query, get_elements_query};

pub struct MutexServiceAdapter {
  context: Arc<Mutex<MutexContext>>,
  command: CommandBus,
  query: QueryBus,
}

impl MutexServiceAdapter {
  pub fn new(context: Arc<Mutex<MutexContext>>, command: CommandBus, query: QueryBus) -> Self {
    Self {
      context,
      command,
      query,
    }
  }
}

#[async_trait]
impl MutexServicePort for MutexServiceAdapter {
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
