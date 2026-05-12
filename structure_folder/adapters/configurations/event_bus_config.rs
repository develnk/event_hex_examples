use event_hex::application::ports::domain_event_handlers::ProjectionUpdaterEventHandlerFactory;
use event_hex::shared_kernel::bus::in_memory::event_bus::{EventBus, EventBusPort};
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::core::identity_access_management::identity::event::access_account_events::AccessAccountEvents;


//Статическое хранилище (используем lazy_static или once_cell)
static EVENT_BUS: OnceCell<Arc<EventBus>> = OnceCell::const_new();
pub async fn event_bus_init() {
    let event_bus = EVENT_BUS.get_or_init(|| async { Arc::new(EventBus::new()) }).await;

    // Регистрируем обработчики событий

    // Для обновления проекций агрегатов, необходимо для каждого доменного события зарегистрировать
    // обработчик обновления проекции.
    event_bus.register_handler::<AccessAccountEvents, _>(ProjectionUpdaterEventHandlerFactory::new()).await;
}

pub fn get_event_bus() -> Arc<EventBus> {
    Arc::clone(EVENT_BUS.get().unwrap())
}
