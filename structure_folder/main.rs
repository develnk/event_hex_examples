use crate::adapters::configurations::command_bus_config::command_bus_init;
use crate::adapters::configurations::db;
use crate::adapters::configurations::event_bus_config::event_bus_init;
use crate::adapters::configurations::query_bus_config::query_bus_init;
use crate::adapters::configurations::service_config::app_state_init;
use crate::adapters::configurations::settings::AppSettings;
use std::error::Error;

mod adapters;
mod application;
mod core;
mod shared_kernel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    start_initialization().await;
    Ok(())
}

async fn start_initialization() {
    dotenvy::dotenv().ok();
    let _ = AppSettings::init();
    db::init().await;

    // Initialization of internal buses
    event_bus_init().await;
    command_bus_init().await;
    query_bus_init().await;
    app_state_init().await;
}