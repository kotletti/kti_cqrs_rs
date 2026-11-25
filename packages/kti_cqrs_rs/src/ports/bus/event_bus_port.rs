use crate::ports::event_port::EventPort;

pub trait EventBusPort: Send + Sync {
    fn send<Event: EventPort + Sync>(&self, event: Event);
}
