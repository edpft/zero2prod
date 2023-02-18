use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{
    configuration::Settings,
    startup,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod", "info", std::io::stdout);
    init_subscriber(subscriber);

    let settings = Settings::get().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy_with(settings.database.with_db());

    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port,
    );
    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection_pool)?.await
}
