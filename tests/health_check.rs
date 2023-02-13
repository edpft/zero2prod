use std::net::TcpListener;

use reqwest::Client;

use sqlx::PgPool;
use zero2prod::{configuration::Settings, startup};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// Launch application in background
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let settings = Settings::get().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let server = startup::run(listener, connection_pool.clone()).expect("Failed to bind address.");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

// /health_check returns a `200 OK` response with no body
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urleoncoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email."),
        ("email=ursula_le_guin%40gmail.com", "missing the name."),
        ("", "missing both name and email."),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urleoncoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
