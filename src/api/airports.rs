use crate::api::client::ApiClient;
use crate::models::airport::Airport;
use serde_json::json;

pub async fn get_airports(client: &ApiClient) -> Result<Vec<Airport>, reqwest::Error> {
    let resp = client.get("/airports").await?;
    let airports: Vec<Airport> = resp.json().await?;
    Ok(airports)
}

pub async fn create_airport(client: &ApiClient, airport: &Airport) -> Result<Airport, reqwest::Error> {
    let resp = client.post("/airports", &json!(airport)).await?;
    let created: Airport = resp.json().await?;
    Ok(created)
}

pub async fn update_airport(client: &ApiClient, id: i32, airport: &Airport) -> Result<Airport, reqwest::Error> {
    let resp = client.put(&format!("/airports/{}", id), &json!(airport)).await?;
    let updated: Airport = resp.json().await?;
    Ok(updated)
}

pub async fn delete_airport(client: &ApiClient, id: i32) -> Result<(), reqwest::Error> {
    client.delete(&format!("/airports/{}", id)).await?;
    Ok(())
}