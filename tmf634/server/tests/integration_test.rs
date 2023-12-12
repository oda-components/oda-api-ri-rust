#[path = "../src/server.rs"]
mod server;
use crate::server::create;
use oda_sdk_tmf634::models;
use tokio::test;
use reqwest;
use serde_json;

#[test]
async fn get_resource_specification() {
    let url = "http://localhost:8080/tmf-api/resourceCatalog/v4/resourceSpecification";
    // Start the server
    let server_handle = tokio::spawn(async {
        let redis = "redis://127.0.0.1";
        let bind = "0.0.0.0";
        let port = "8080";

        create(redis, bind, port, false).await;
    });
    // Create a client
    let client = reqwest::Client::new();
    // Make a GET /resourceSpecification request to the server
    let get_spec_resp = client
        .get(url)
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");
    // Printout raw and decoded JSON
    println!("GET /resourceSpecification JSON raw response: {:?}", get_spec_resp);
    println!("GET /resourceSpecification decoded response: {:?}", serde_json::from_str::<Vec<models::ResourceSpecification>>(&get_spec_resp));
    // Stop the server
    server_handle.abort();
}

#[test]
async fn post_resource_specification() {
    let url = "http://localhost:8080/tmf-api/resourceCatalog/v4/resourceSpecification";
    // Start the server
    let server_handle = tokio::spawn(async {
        let redis = "redis://127.0.0.1";
        let bind = "0.0.0.0";
        let port = "8080";

        create(redis, bind, port, false).await;
    });
    // Create a client
    let client = reqwest::Client::new();
    // POST for create new spec
    let post_spec_resp = client
        .post(url)
        .body(r#"{"name": "example"}"#)
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");
    // Decode POST response
    let spec_post_entity = serde_json::from_str::<models::ResourceSpecification>(&post_spec_resp);
    println!("POST /resourceSpecification JSON raw response: {:?}", post_spec_resp);
    println!("POST /resourceSpecification JSON decoded response: {:?}",  spec_post_entity);
    // Fetch ID of new spec from decoded response body of POST request
    let spec_id: String = spec_post_entity.map(|res| res.id.unwrap_or_default()).unwrap_or_default();
    // GET created spec
    let get_new_spec_resp = client
        .get(url.to_owned() + "/" + &spec_id)
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");
    // Compare spec from GET and POST requests
    assert_eq!(get_new_spec_resp, post_spec_resp);
    // Stop the server
    server_handle.abort();
}

#[test]
async fn delete_resource_specification() {
    let url = "http://localhost:8080/tmf-api/resourceCatalog/v4/resourceSpecification";
    // Start the server
    let server_handle = tokio::spawn(async {
        let redis = "redis://127.0.0.1";
        let bind = "0.0.0.0";
        let port = "8080";

        create(redis, bind, port, false).await;
    });
    // Create a client
    let client = reqwest::Client::new();
    // POST for create new spec
    let post_spec_resp = client
        .post(url)
        .body(r#"{"name": "example"}"#)
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");
    // Decode POST response
    let spec_post_entity = serde_json::from_str::<models::ResourceSpecification>(&post_spec_resp);
    // Fetch ID of new spec from decoded response body of POST request
    let spec_id: String = spec_post_entity.map(|res| res.id.unwrap_or_default()).unwrap_or_default();
    // DELETE spec and get status code - expected 204
    let del_spec_status = client
        .delete(url.to_owned() + "/" + &spec_id)
        .send()
        .await
        .unwrap()
        .status();
    // Compare HTTP status code from DELETE request
    assert_eq!(del_spec_status, 204);
    // GET deleted spec - expected now 404
    let get_deleted_spec_status = client
        .get(url.to_owned() + "/" + &spec_id)
        .send()
        .await
        .unwrap()
        .status();
    // Compare GET HTTP status code for deleted spec
    assert_eq!(get_deleted_spec_status, 404);
    // Stop the server
    server_handle.abort();
}
