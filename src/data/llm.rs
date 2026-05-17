use std::sync::{Arc, RwLock};

use futures_util::StreamExt;
use reqwest::Client;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Clone)]
pub struct LLM {
    client: Client,
    api_key: String,
    pub history: Arc<RwLock<Vec<Message>>>,
    max_history: usize,
    endpoint: String,
    pub llm_output: Arc<RwLock<String>>,
}

impl LLM {
    pub fn new(api_key: String, endpoint: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            history: Arc::new(RwLock::new(vec![Message {
                role: "system".to_string(),
                content: "你是一个编程助手".to_string(),
            }])),
            max_history: 12,
            endpoint,
            llm_output: Arc::new(RwLock::new(String::new())),
        }
    }

    pub fn clear(&mut self) {
        self.history.write().unwrap().truncate(1);
    }

    pub fn trim(&mut self) {
        while self.history.read().unwrap().len() > self.max_history {
            self.history.write().unwrap().remove(1);
        }
    }

    pub async fn chat(&mut self, input: String) {
        {
            let mut h = self.history.write().unwrap();
            h.push(Message {
                role: "user".to_string(),
                content: input,
            });
        }

        let body = RequestBody {
            model: "llm".to_string(),
            messages: self.history.read().unwrap().clone(),
            stream: true,
        };

        let result = self
            .client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .unwrap();

        let mut stream = result.bytes_stream();

        let mut reply = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.unwrap();

            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        self.history.write().unwrap().push(Message {
                            role: "assistant".to_string(),
                            content: reply.clone(),
                        });
                        return;
                    }

                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data)
                        && let Some(content) = json["choices"][0]["delta"]["content"].as_str()
                    {
                        self.llm_output.write().unwrap().push_str(content);
                        reply.push_str(content);

                        std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    }
                }
            }
        }
    }
}
