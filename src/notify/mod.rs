use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use rocket::serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

static BASE_URL: &str = "https://api.notifications.service.gov.uk";

pub struct NotifyClient {
    api_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    iss: String,
    iat: usize,
}

impl NotifyClient {
    pub fn new(api_key: String) -> Self {
        NotifyClient { api_key }
    }

    pub async fn send_email(
        &self,
        email_address: &String,
        template_id: String,
        personalisation: Map<String, Value>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();

        let token = self.create_jwt().unwrap();
        let auth_header: &str = &["Bearer ", token.as_str()].concat();

        let mut body = Map::new();
        body.insert(
            "email_address".to_string(),
            Value::String(email_address.clone()),
        );
        body.insert("template_id".to_string(), Value::String(template_id));
        body.insert(
            "personalisation".to_string(),
            Value::Object(personalisation),
        );

        client
            .post(BASE_URL.to_owned() + "/v2/notifications/email")
            .header(USER_AGENT, "rust-client-pre-alpha")
            .header(AUTHORIZATION, auth_header)
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
    }

    fn create_jwt(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims {
            iss: String::from(self.service_id()),
            iat: Utc::now().timestamp() as usize,
        };
        let header = Header::new(Algorithm::HS256);
        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.secret_key()),
        )
    }

    fn service_id(&self) -> &str {
        &self.api_key[(self.api_key.len() - 73)..=(self.api_key.len() - 38)]
    }

    fn secret_key(&self) -> &[u8] {
        let key = &self.api_key[(self.api_key.len() - 36)..self.api_key.len()];
        key.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_id() {
        let client = NotifyClient {
            api_key: String::from("my_test_key-26785a09-ab16-4eb0-8407-a37497a57506-3d844edf-8d35-48ac-975b-e847b4f122b0")
        };
        let service_id = client.service_id();
        assert_eq!(service_id, "26785a09-ab16-4eb0-8407-a37497a57506");
    }

    #[test]
    fn secret_key() {
        let client = NotifyClient {
            api_key: String::from("my_test_key-26785a09-ab16-4eb0-8407-a37497a57506-3d844edf-8d35-48ac-975b-e847b4f122b0")
        };
        let secret_key = client.secret_key();
        assert_eq!(secret_key, b"3d844edf-8d35-48ac-975b-e847b4f122b0");
    }
}
