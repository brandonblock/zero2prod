use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[actix_web::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    //arrange
    let test_app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_confirmation_links(&email_request);

    //act
    let response = reqwest::get(confirmation_links.html).await.unwrap();

    // assert
    assert_eq!(response.status().as_u16(), 200);
}

#[actix_web::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    //arrange
    let test_app = spawn_app().await;

    //act
    let response = reqwest::get(&format!("{}/subscriptions/confirm", test_app.address))
        .await
        .unwrap();

    //assert
    assert_eq!(response.status().as_u16(), 400);
}
