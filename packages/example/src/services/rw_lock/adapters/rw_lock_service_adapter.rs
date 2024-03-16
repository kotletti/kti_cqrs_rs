use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_rs::core::bus::{command_bus::CommandBus, query_bus::QueryBus};
use tokio::sync::RwLock;

use crate::services::rw_lock::commands::{add_element_command, remove_element_command};
use crate::services::rw_lock::contexts::rw_lock_context::RwLockContext;
use crate::services::rw_lock::ports::rw_lock_service_port::RwLockServicePort;
use crate::services::rw_lock::queries::{get_count_elements_query, get_elements_query};

pub struct RwLockServiceAdapter {
  context: Arc<RwLock<RwLockContext>>,
  command: CommandBus,
  query: QueryBus,
}

impl RwLockServiceAdapter {
  pub fn new(context: Arc<RwLock<RwLockContext>>, command: CommandBus, query: QueryBus) -> Self {
    Self {
      context,
      command,
      query,
    }
  }
}

#[async_trait]
impl RwLockServicePort for RwLockServiceAdapter {
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
