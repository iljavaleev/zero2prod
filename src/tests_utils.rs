use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

use crate::configuration::{DatabaseSettings, get_configuration};
use crate::startup::run;
use crate::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// cargo test -- --nocapture
pub async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // create random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    // get address with random port for return
    let address = format!("http://127.0.0.1:{}", port);

    let mut conf = get_configuration().expect("Failed to read configuration.");
    // create random db name for tests
    conf.database.database_name = Uuid::new_v4().to_string();

    let connection = configure_database(&conf.database).await;

    let server = run(listener, connection.clone()).expect("Failed to bind address");

    // takes a future and hands it over to the runtime for polling, without waiting for its completion
    // tokio::test spins up a new runtime at the beginning of each test case

    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(config.connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

    let query: &'static str =
        Box::leak(format!(r#"CREATE DATABASE "{}";"#, config.database_name).into_boxed_str());

    connection
        .execute(query)
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
