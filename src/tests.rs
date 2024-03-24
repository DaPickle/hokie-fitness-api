use crate::error::Result;
#[cfg(test)]
use axum_test::TestServer;
#[cfg(test)]
use axum_test::TestServerConfig;

#[cfg(test)]
async fn new_test_app() -> Result<TestServer> {
    use crate::{model::ModelController, new_app};

    let mc = ModelController::new();

    let app = new_app(mc);
    let config = TestServerConfig::builder()
        // Preserve cookies across requests
        // for the session cookie to work.
        .save_cookies()
        .mock_transport()
        .build();

    Ok(TestServer::new_with_config(app, config).unwrap())
}


use ::serde_json::json;

#[tokio::test]
async fn it_should_create_session_on_login() {
    let Ok(server) = new_test_app().await else {
        panic!("Womp Womp")
    };

    let res = server.get("/api/calculateCalories").json(&json!({
        "activity": "moderate",
        "gender": "male",
        "height": 1.85928,
        "weight": 74.3,
        "age": 51,
    })).await;

    println!();
    println!("{res:?}");
}