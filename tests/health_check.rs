use std::net::TcpListener;


// `tokio::test` is the testing equivalent of `tokio::main`.
// it also spares you from having to specify the `#[test]` attribute
//
// you can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // arrange
    let address = spawn_app();

    // we need to bring in `reqwest`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// launch our application in the background ~somehow~
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to address");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind to address");

    // launch the server as a background task
    // tokio::spawn returns a handle to the spawned future
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);

    // we return the application address to the caller
    format!("http://127.0.0.1:{}", port)
}
