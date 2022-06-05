use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

const BASE_URL: &'static str = "";

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRespone {
    pub data: User,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub token: String,
}

pub async fn createAccount(username: String, password: String) -> LoginRespone {
    Request::post(&format!("{}/createaccount", BASE_URL))
        .header("Content-Type", "application/json")
        .body(
            json!({
            "username":username,
            "password":password,
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap()
        .json::<LoginRespone>()
        .await
        .unwrap()
}

pub async fn login(username: String, password: String) -> LoginRespone {
    Request::post(&format!("{}/login", BASE_URL))
        .header("Content-Type", "application/json")
        .body(
            json!({
            "username":username,
            "password":password,
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap()
        .json::<LoginRespone>()
        .await
        .unwrap()
}
