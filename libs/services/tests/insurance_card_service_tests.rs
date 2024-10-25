#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, POST};
    use httpmock::MockServer;
    use models::patient_profile::{
        InsuranceCard, InsuranceCardForCreate, InsuranceCardImage, InsuranceCardImageForCreate,
    };
    use patient_profile::InsuranceCardService;
    use serial_test::serial;
    use services::prelude::*;
    use services::*;

    fn get_mock_insurance_card(rank: i32) -> InsuranceCard {
        InsuranceCard {
            rank,
            images: vec![
                InsuranceCardImage {
                    side: 1,
                    url: "https://s3-bucket.s3.amazonaws.com/insurance_card_images/1234/front.png"
                        .to_string(),
                    ttl: 3600,
                },
                InsuranceCardImage {
                    side: 2,
                    url: "https://s3-bucket.s3.amazonaws.com/insurance_card_images/1234/back.png"
                        .to_string(),
                    ttl: 3600,
                },
            ],
        }
    }

    //#[serial]
    //#[tokio::test]
    //async fn test_find_insurance_cards_success() {
    //    // Start a local mock server
    //    let server = MockServer::start_async().await;
    //
    //    // Set the mock env
    //    std::env::set_var("TEST_ENV", "TRUE");
    //    std::env::set_var("MOCK_SERVER_URL", server.base_url());
    //
    //    let insurance_card_rank = 1;
    //    let insurance_card = get_mock_insurance_card(insurance_card_rank);
    //
    //    let mock = server.mock(|when, then| {
    //        when.method(GET)
    //            .path(format!("/insurance_cards/", insurance_card_rank));
    //        then.status(200)
    //            .header("Content-Type", "application/json")
    //            .body(serde_json::to_string(&insurance_card).unwrap());
    //    });
    //
    //    // Create a client pointing to the mock server
    //    let client = Client::new().await.unwrap();
    //    let insurance_card_service = InsuranceCardService::new(&client);
    //
    //    // Call the method under test
    //    let result = insurance_card_service.find(insurance_card_rank).await;
    //
    //    println!("result: {result:#?}");
    //
    //    // Assert the result
    //    assert!(result.is_ok());
    //    let fetched_insurance_card = result.unwrap();
    //    assert_eq!(fetched_insurance_card.rank, insurance_card_rank);
    //    assert_eq!(
    //        fetched_insurance_card.images[0].url,
    //        "https://s3-bucket.s3.amazonaws.com/insurance_card_images/1234/front.png"
    //    );
    //
    //    // Ensure the mock was called
    //    mock.assert_async().await;
    //}

    #[serial]
    #[tokio::test]
    async fn test_create_insurance_card_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let insurance_card_rank = 1;
        let insurance_card_for_create = InsuranceCardForCreate {
            rank: insurance_card_rank,
            images: vec![
                InsuranceCardImageForCreate {
                    side: 1,
                    url: "https://s3-bucket.s3.amazonaws.com/insurance_card_images/1234/front.png"
                        .to_string(),
                    ttl: 3600,
                },
                InsuranceCardImageForCreate {
                    side: 2,
                    url: "https://s3-bucket.s3.amazonaws.com/insurance_card_images/1234/back.png"
                        .to_string(),
                    ttl: 3600,
                },
            ],
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/insurance_cards")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&insurance_card_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&get_mock_insurance_card(insurance_card_rank)).unwrap(),
                );
        });

        let client = Client::new().await.unwrap();
        let service = InsuranceCardService::new(&client);

        let result = service.post(&insurance_card_for_create).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let created_insurance_card = result.unwrap();
        assert_eq!(created_insurance_card.rank, insurance_card_rank);

        mock.assert_async().await;
    }

    //#[serial]
    //#[tokio::test]
    //async fn test_update_patch_insurance_card_success() {
    //    let server = MockServer::start_async().await;
    //
    //    std::env::set_var("TEST_ENV", "TRUE");
    //    std::env::set_var("MOCK_SERVER_URL", server.base_url());
    //
    //    let insurance_card_rank = 1;
    //    let mock_insurance_card = InsuranceCard {
    //        rank: insurance_card_rank,
    //        images: vec![
    //            InsuranceCardImage {
    //                side: 1,
    //                url: "https://s3-bucket.s3.amazonaws.com/insurance_card_images/1234/updated_front.png".to_owned(),
    //                ttl: 3600,
    //            },
    //            InsuranceCardImage {
    //                side: 2,
    //                url: "https://s3-bucket.s3.amazonaws.com/insurance_card_images/1234/back.png".to_owned(),
    //                ttl: 3600,
    //            },
    //        ],
    //    };
    //
    //    let mock = server.mock(|when, then| {
    //        when.method(PATCH)
    //            .path(format!("/insurance_cards/{}/", insurance_card_rank))
    //            .header("Content-Type", "application/json")
    //            .json_body_partial(
    //                r#"{
    //                    "images": [{
    //                        "side": 1,
    //                        "url": "https://s3-bucket.s3.amazonaws.com/insurance_card_images/1234/updated_front.png"
    //                    }]
    //                }"#,
    //            );
    //        then.status(200)
    //            .header("Content-Type", "application/json")
    //            .body(serde_json::to_string(&mock_insurance_card).unwrap());
    //    });
    //
    //    let client = Client::new().await.unwrap();
    //    let insurance_card_service = InsuranceCardService::new(&client);
    //
    //    let insurance_card_fu = InsuranceCardForUpdate {
    //        images: Some(vec![InsuranceCardImageForUpdate {
    //            side: Some(1),
    //            url: Some("https://s3-bucket.s3.amazonaws.com/insurance_card_images/1234/updated_front.png".to_owned()),
    //            ttl: None,
    //        }]),
    //        ..InsuranceCardForUpdate::default()
    //    };
    //
    //    let result = insurance_card_service
    //        .patch(insurance_card_rank, &insurance_card_fu)
    //        .await;
    //
    //    println!("result: {result:#?}");
    //
    //    assert!(result.is_ok());
    //    let updated_insurance_card = result.unwrap();
    //    assert_eq!(
    //        updated_insurance_card.images[0].url,
    //        "https://s3-bucket.s3.amazonaws.com/insurance_card_images/1234/updated_front.png"
    //    );
    //
    //    mock.assert_async().await;
    //}

    #[serial]
    #[tokio::test]
    async fn test_delete_insurance_card_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let insurance_card_rank = 1;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/insurance_cards/{}/", insurance_card_rank));
            then.status(204); // No Content
        });

        let client = Client::new().await.unwrap();
        let insurance_card_service = InsuranceCardService::new(&client);

        let result = insurance_card_service.delete(insurance_card_rank).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
