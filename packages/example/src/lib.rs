pub mod adapters;
pub mod commands;
pub mod contexts;
pub mod events;
pub mod ports;
pub mod queries;
pub mod stores;

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use kti_cqrs_rs::{
        adapters::{
            command_bus_adapter::CommandBusAdapter, event_bus_adapter::EventBusAdapter,
            query_bus_adapter::QueryBusAdapter,
        },
        errors::error::Error,
    };
    use tokio::{sync::RwLock, time::sleep};

    use crate::{
        adapters::{
            element_repository_command_adapter::ElementRepositoryCommandAdapter,
            element_repository_query_adapter::ElementRepositoryQueryAdapter,
            element_service_adapter::ElementServiceAdapter,
        },
        commands::{
            add_element_command::AddElementHandler, remove_element_command::RemoveElementHandler,
            update_element_command::UpdateElementHandler,
        },
        events::increment_element_event::IncrementElementHandler,
        ports::element_service_port::ElementServicePort,
        queries::{
            get_count_elements_query::GetCountElementsHandler,
            get_elements_query::GetElementsHandler,
        },
        stores::element_repository_store::ElementRepositoryStore,
    };

    fn create_service() -> Result<Box<dyn ElementServicePort>, Error> {
        let store = ElementRepositoryStore::new(Arc::new(RwLock::new(vec![])));

        let query_repository = Arc::new(ElementRepositoryQueryAdapter::new(store.clone()));
        let command_repository = Arc::new(ElementRepositoryCommandAdapter::new(store));

        let command_bus = CommandBusAdapter::default()
            .register(AddElementHandler::new(command_repository.clone()))?
            .register(UpdateElementHandler::new(command_repository.clone()))?
            .register(RemoveElementHandler::new(command_repository.clone()))?;

        let query_bus = QueryBusAdapter::default()
            .register(GetElementsHandler::new(query_repository.clone()))?
            .register(GetCountElementsHandler::new(query_repository))?;

        let event_bus = EventBusAdapter::default()
            .register(IncrementElementHandler::new(command_repository))?;

        Ok(Box::new(ElementServiceAdapter::new(
            command_bus,
            query_bus,
            event_bus,
        )))
    }

    #[tokio::test]
    async fn should_be_empty_vector() -> Result<(), Error> {
        let service = create_service()?;

        let count = service.get_count().await?;

        assert_eq!(count, 0);

        Ok(())
    }

    #[tokio::test]
    async fn should_be_one_element() -> Result<(), Error> {
        let service = create_service()?;

        service.add_element(1).await?;

        let count = service.get_count().await?;

        assert_eq!(count, 1);

        Ok(())
    }

    #[tokio::test]
    async fn should_be_empty_after_remove() -> Result<(), Error> {
        let service = create_service()?;

        service.add_element(1).await?;
        service.remove_element(1).await?;

        let count = service.get_count().await?;

        assert_eq!(count, 0);

        Ok(())
    }

    #[tokio::test]
    async fn should_be_error_on_remove_not_existed_element() -> Result<(), Error> {
        let service = create_service()?;

        let res = service.remove_element(1).await;

        assert!(res.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn should_be_updated_element() -> Result<(), Error> {
        let from_element = 1;
        let to_element = 2;
        let service = create_service()?;

        service.add_element(from_element).await?;
        service.update_element(from_element, to_element).await?;

        let elements = service.get_elements().await?;

        let updated_element = *elements.first().ok_or("Element not found")?;

        assert_eq!(updated_element, to_element);

        Ok(())
    }

    #[tokio::test]
    async fn should_be_incremented_by_event() -> Result<(), Error> {
        let element = 43;
        let service = create_service()?;

        service.add_element_with_event(element).await?;

        sleep(Duration::from_secs(1)).await;

        let elements = service.get_elements().await?;

        let incremented_element = *elements.first().ok_or("Element not found")?;

        assert_eq!(incremented_element, element + 1);

        Ok(())
    }

    #[tokio::test]
    async fn should_be_failed_incremented_by_event() -> Result<(), Error> {
        let element = 42;
        let service = create_service()?;

        service.add_element_with_event(element).await?;

        sleep(Duration::from_secs(1)).await;

        let elements = service.get_elements().await?;

        let incremented_element = *elements.first().ok_or("Element not found")?;

        assert_eq!(incremented_element, element);

        Ok(())
    }
}
