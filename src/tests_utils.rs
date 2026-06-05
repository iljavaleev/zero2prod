use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

use crate::configuration::{DatabaseSettings, get_configuration};
use crate::startup::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
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
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    let query: &'static str =
        Box::leak(format!(r#"CREATE DATABASE "{}";"#, config.database_name).into_boxed_str());

    connection
        .execute(query)
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
