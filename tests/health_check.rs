//! tests/health_check.rs
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}


async fn spawn_app() -> TestApp{
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);


    let conf = get_configuration().expect("Failed to read configuration.");
    let connection = 
        PgPool::connect(&conf.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");


    let server = run(listener, connection.clone())
        .expect("Failed to bind address");
    
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection,
    }
}


#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &test_app.await.address))
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

