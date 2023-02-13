use secrecy::ExposeSecret;
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
    let connection_pool = PgPool::connect(settings.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", settings.application_port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection_pool)?.await
}
