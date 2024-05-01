use crate::{
    models::{ApiResponse, AppState, ResolveQuery},
    utils::{get_error, hash_domain},
};
use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
};
use chrono::{Duration, Utc};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use starknet::core::{
    crypto::{ecdsa_sign, pedersen_hash},
    types::FieldElement,
};
use starknet_id::encode;
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref FIELD_STARKNET: FieldElement = FieldElement::from_dec_str("8319381555716711796").unwrap();
    static ref HASH_NAME: FieldElement = FieldElement::from_dec_str("2216433769979142660313091089524420126762954343").unwrap();
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ResolveQuery>,
) -> impl IntoResponse {
    let url = format!(
        "https://api.notion.com/v1/databases/{}/query",
        state.conf.notion.database_id
    );
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Notion-Version", HeaderValue::from_static("2022-06-28"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let token = format!("Bearer {}", state.conf.notion.secret.clone());
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());

    // Create the JSON payload
    let payload = json!({
        "filter": {
            "property": "Domain",
            "title": {
                "equals": format!("{}.notion.stark", query.domain)
            }
        }
    });

    match client
        .post(&url)
        .headers(headers)
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => match response.json::<ApiResponse>().await {
            Ok(res) => {
                if let Some(first_result) = res.results.first() {
                    if let Some(first_rich_text) = first_result.properties.Address.rich_text.first()
                    {
                        let address = &first_rich_text.plain_text;

                        let domain_splitted: Vec<&str> = query.domain.split('.').collect();
                        let encoded_domain: Vec<FieldElement> = domain_splitted
                            .iter()
                            .map(|part| encode(part).unwrap())
                            .collect();
                        let hashed_domain = hash_domain(encoded_domain);

                        let max_validity = Utc::now() + Duration::hours(1);
                        let max_validity_seconds = max_validity.timestamp();
                        let hash = pedersen_hash(
                            &pedersen_hash(
                                &pedersen_hash(
                                    &pedersen_hash(
                                        &HASH_NAME,
                                        &FieldElement::from_dec_str(
                                            max_validity_seconds.to_string().as_str(),
                                        )
                                        .unwrap(),
                                    ),
                                    &hashed_domain,
                                ),
                                &FIELD_STARKNET,
                            ),
                            &FieldElement::from_hex_be(address).unwrap(),
                        );

                        match ecdsa_sign(&state.conf.starknet.private_key, &hash) {
                            Ok(signature) => (
                                StatusCode::OK,
                                Json(
                                    json!({"address": address, "r": signature.r, "s": signature.s, "max_validity": max_validity_seconds}),
                                ),
                            )
                                .into_response(),
                            Err(e) => get_error(format!("Error while generating signature: {}", e)),
                        }
                    } else {
                        get_error("No address found for this domain".to_string())
                    }
                } else {
                    get_error("Domain not found".to_string())
                }
            }
            Err(e) => get_error(format!(
                "Error while decoding responses from Notion api: {}",
                e
            )),
        },
        Err(e) => get_error(format!("Error while fetching Notion api: {}", e)),
    }
}
