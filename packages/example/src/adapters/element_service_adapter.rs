use async_trait::async_trait;
use kti_cqrs_rs::adapters::command_bus_adapter::CommandBusAdapter;
use kti_cqrs_rs::adapters::event_bus_adapter::EventBusAdapter;
use kti_cqrs_rs::adapters::query_bus_adapter::QueryBusAdapter;
use kti_cqrs_rs::errors::error::Error;
use kti_cqrs_rs::ports::bus::command_bus_port::CommandBusPort;
use kti_cqrs_rs::ports::bus::event_bus_port::EventBusPort;
use kti_cqrs_rs::ports::bus::query_bus_port::QueryBusPort;

use crate::commands::add_element_command::AddElementCommand;
use crate::commands::remove_element_command::RemoveElementCommand;
use crate::commands::update_element_command::UpdateElementCommand;
use crate::events::increment_element_event::IncrementElementEvent;
use crate::ports::element_service_port::ElementServicePort;
use crate::queries::get_count_elements_query::GetCountElementsQuery;
use crate::queries::get_elements_query::GetElementsQuery;

pub struct ElementServiceAdapter {
    command: CommandBusAdapter,
    query: QueryBusAdapter,
    event: EventBusAdapter,
}

impl ElementServiceAdapter {
    pub fn new(command: CommandBusAdapter, query: QueryBusAdapter, event: EventBusAdapter) -> Self {
        Self {
            command,
            query,
            event,
        }
    }
}

#[async_trait]
impl ElementServicePort for ElementServiceAdapter {
    async fn get_elements(&self) -> Result<Vec<i32>, Error> {
        self.query.send(GetElementsQuery).await
    }

    async fn add_element(&self, element: i32) -> Result<(), Error> {
        self.command.send(AddElementCommand { element }).await
    }

    async fn add_element_with_event(&self, element: i32) -> Result<(), Error> {
        self.command.send(AddElementCommand { element }).await?;

        if element > 42 {
            self.event.send(IncrementElementEvent { element });
        }

        Ok(())
    }

    async fn remove_element(&self, element: i32) -> Result<(), Error> {
        self.command.send(RemoveElementCommand { element }).await
    }

    async fn update_element(&self, from_element: i32, to_element: i32) -> Result<(), Error> {
        self.command
            .send(UpdateElementCommand {
                from_element,
                to_element,
            })
            .await
    }

    async fn get_count(&self) -> Result<usize, Error> {
        self.query.send(GetCountElementsQuery).await
    }
}
