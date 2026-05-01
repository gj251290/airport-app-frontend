use crate::api::client::ApiClient;
use crate::models::airline::Airlinie;
use serde_json::json;

pub async fn get_airlines(client: &ApiClient) -> Result<Vec<Airline>, reqwest::Error> {
    let resp = client.get("/airlines").await?;
    let airlines: Vec<Airline> = resp.json().await?;
    Ok(airlines)
}

pub async fn create_airline(client: &ApiClient, airline: &Airline) -> Result<Airline, reqwest::Error> {
    let resp = client.post("/airlines", &json!(airline)).await?;
    let created: Airline = resp.json().await?;
    Ok(created)
}

pub async fn update_airline(client: &ApiClient, id: i32, airline: &Airline) -> Result<Airline, reqwest::Error> {
    let resp = client.put(&format!("/airlines/{}", id), &json!(airline)).await?;
    let updated: Airline = resp.json().await?;
    Ok(updated)
}

pub async fn delete_airline(client: &ApiClient, id: i32) -> Result<(), reqwest::Error> {
    client.delete(&format!("/airlines/{}", id)).await?;
    Ok(())
}