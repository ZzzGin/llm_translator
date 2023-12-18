use crate::models::gpt::{ApiResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue, InvalidHeaderValue},
    Client,
};
use std::env;

pub async fn call(
    messages: Vec<Message>,
) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API Key
    let api_key: String = env::var("OPEN_AI_KEY")
        .expect("OPEN_AI_KEY not found in environment variables");
    let org_key: String = env::var("OPEN_AI_ORG")
        .expect("OPEN_AI_ORG not found in environment variables");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    let mut headers: HeaderMap = HeaderMap::new();

    // Create API Key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).map_err(
            |e: InvalidHeaderValue| -> Box<dyn std::error::Error + Send> {
                Box::new(e)
            },
        )?,
    );

    // Create Open AI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(org_key.as_str()).map_err(
            |e: InvalidHeaderValue| -> Box<dyn std::error::Error + Send> {
                Box::new(e)
            },
        )?,
    );

    // Create client
    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e: reqwest::Error| -> Box<dyn std::error::Error + Send> {
            Box::new(e)
        })?;

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1,
    };

    /*
    Troubleshooting
    let res_raw: Response = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();
    dbg!(res_raw.text().await.unwrap());
    */

    // Extract API Response
    let res: ApiResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e: reqwest::Error| -> Box<dyn std::error::Error + Send> {
            Box::new(e)
        })?
        .json()
        .await
        .map_err(|e: reqwest::Error| -> Box<dyn std::error::Error + Send> {
            Box::new(e)
        })?;
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_to_openai() {
        let message: Message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response."
                .to_string(),
        };

        let res: Result<String, Box<dyn std::error::Error + Send>> =
            call(vec![message]).await;
        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}
