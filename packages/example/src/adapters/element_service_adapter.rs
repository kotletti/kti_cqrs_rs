use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::adapters::command_bus_adapter::CommandBusAdapter;
use kti_cqrs_rs::adapters::event_bus_adapter::EventBusAdapter;
use kti_cqrs_rs::adapters::query_bus_adapter::QueryBusAdapter;
use kti_cqrs_rs::errors::error::Error;
use kti_cqrs_rs::ports::bus::command_bus_port::CommandBusPort;
use kti_cqrs_rs::ports::bus::event_bus_port::EventBusPort;
use kti_cqrs_rs::ports::bus::query_bus_port::QueryBusPort;
use tokio::sync::RwLock;

use crate::commands::add_element_command::AddElementCommand;
use crate::commands::remove_element_command::RemoveElementCommand;
use crate::commands::update_element_command::UpdateElementCommand;
use crate::contexts::app_context::AppContext;
use crate::events::increment_element_event::IncrementElementEvent;
use crate::ports::element_service_port::ElementServicePort;
use crate::queries::get_count_elements_query::GetCountElementsQuery;
use crate::queries::get_elements_query::GetElementsQuery;

pub struct ElementServiceAdapter {
  context: Arc<RwLock<AppContext>>,
  command: CommandBusAdapter,
  query: QueryBusAdapter,
  event: EventBusAdapter,
}

impl ElementServiceAdapter {
  pub fn new(
    context: Arc<RwLock<AppContext>>,
    command: CommandBusAdapter,
    query: QueryBusAdapter,
    event: EventBusAdapter,
  ) -> Self {
    Self {
      context,
      command,
      query,
      event,
    }
  }
}

#[async_trait]
impl ElementServicePort for ElementServiceAdapter {
  async fn get_elements(&self) -> Result<Vec<i32>, Error> {
    let query = GetElementsQuery;

    let elements = self
      .query
      .send(Box::new(query), self.context.clone())
      .await?;

    Ok(elements)
  }

  async fn add_element(&self, element: i32) -> Result<(), Error> {
    let command = AddElementCommand { element };

    self
      .command
      .send(Box::new(command), self.context.clone())
      .await
  }

  async fn add_element_with_event(&self, element: i32) -> Result<(), Error> {
    let command = AddElementCommand { element };

    self
      .command
      .send(Box::new(command), self.context.clone())
      .await?;

    if element > 42 {
      let event = IncrementElementEvent { element };

      self.event.send(Box::new(event), self.context.clone());
    }

    Ok(())
  }

  async fn remove_element(&self, element: i32) -> Result<(), Error> {
    let command = RemoveElementCommand { element };

    self
      .command
      .send(Box::new(command), self.context.clone())
      .await
  }

  async fn update_element(&self, from_element: i32, to_element: i32) -> Result<(), Error> {
    let command = UpdateElementCommand {
      from_element,
      to_element,
    };

    self
      .command
      .send(Box::new(command), self.context.clone())
      .await
  }

  async fn get_count(&self) -> Result<usize, Error> {
    let query = GetCountElementsQuery;

    let count = self
      .query
      .send(Box::new(query), self.context.clone())
      .await?;

    Ok(count)
  }
}
