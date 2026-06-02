//! tests/health_check.rs
use zero2prod::tests_utils::spawn_app;

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
