use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};
use llmservice_flows::{chat::ChatOptions, LLMServiceFlows};
use serde_json::Value;
use std::collections::HashMap;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv::dotenv().ok();
    logger::init();
    request_received(handler).await;
}

async fn handler(_qry: HashMap<String, Value>, body: Vec<u8>) {
    let co = ChatOptions {
        model: Some("gpt-4"),
        token_limit: 8192,
        ..Default::default()
    };

    let mut lf = LLMServiceFlows::new("https://api.openai.com/v1");
    let api_key = std::env::var("api_key").unwrap();

    lf.set_api_key(&api_key);

    let r = match lf
        .chat_completion(
            "any_conversation_id",
            String::from_utf8_lossy(&body).into_owned().as_str(),
            &co,
        )
        .await
    {
        Ok(c) => c.choice,
        Err(e) => e,
    };

    send_response(
        200,
        vec![(
            String::from("content-type"),
            String::from("text/plain; charset=UTF-8"),
        )],
        r.as_bytes().to_vec(),
    );
}
