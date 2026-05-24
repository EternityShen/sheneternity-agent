use std::{
    fs::read_to_string,
    sync::{Arc, RwLock},
};

use futures_util::StreamExt;

use reqwest::Client;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,

    #[serde(rename = "type")]
    pub kind: String,

    pub function: ToolFunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunctionCall {
    pub name: String,
    pub arguments: String,
}

pub enum DisplayItem {
    System(String),
    User(String),
    Think(String),
    Chat(String),
    ToolCall(String),
    ToolResult(String),
}

#[derive(Serialize, Clone)]
pub struct Message {
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing, default)]
    pub think_content: Option<String>,
}

impl Message {
    pub fn to_display_items(&self) -> Vec<DisplayItem> {
        let mut items = Vec::new();
        match self.role {
            Role::System => {
                if let Some(c) = &self.content {
                    items.push(DisplayItem::System(format!("[System] {}", c)));
                }
            }

            Role::User => {
                if let Some(c) = &self.content {
                    items.push(DisplayItem::User(c.clone()));
                }
            }

            Role::Assistant => {
                if let Some(t) = &self.think_content {
                    items.push(DisplayItem::Think(t.clone()));
                }
                if let Some(tc) = &self.tool_calls {
                    for call in tc {
                        items.push(DisplayItem::ToolCall(format!("{:?}", call)));
                    }
                }
                if let Some(c) = &self.content {
                    items.push(DisplayItem::Chat(c.clone()));
                }
            }

            Role::Tool => {
                if let Some(c) = &self.content {
                    items.push(DisplayItem::Chat(c.clone()));
                }
            }
        }
        items
    }
}

#[derive(Serialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub kind: String,

    pub function: FunctionDef,
}

#[derive(Serialize)]
pub struct FunctionDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    stream: bool,
}

#[derive(Clone)]
pub struct LLM {
    client: Client,
    api_key: String,
    pub history: Arc<RwLock<Vec<Message>>>,
    max_history: usize,
    endpoint: String,
}

impl LLM {
    pub fn new(api_key: String, endpoint: String) -> Self {
        let prompt = read_to_string("./debug/Prompt.txt").unwrap();
        Self {
            client: Client::new(),
            api_key,
            history: Arc::new(RwLock::new(vec![Message {
                role: Role::System,
                content: Some(prompt),
                tool_calls: None,
                tool_call_id: None,
                think_content: None,
            }])),
            max_history: 12,
            endpoint,
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

    pub async fn chat(&mut self, input: String) -> anyhow::Result<()> {
        {
            let mut h = self.history.write().unwrap();
            h.push(Message {
                role: Role::User,
                content: Some(input),
                tool_call_id: None,
                tool_calls: None,
                think_content: None,
            });
        }

        let body = RequestBody {
            model: "llm".to_string(),
            messages: self
                .history
                .read()
                .unwrap()
                .iter()
                .map(|msg| Message {
                    role: msg.role.clone(),
                    content: msg.content.clone(),
                    tool_calls: msg.tool_calls.clone(),
                    tool_call_id: msg.tool_call_id.clone(),
                    think_content: None,
                })
                .collect(),
            tools: None,
            tool_choice: None,
            stream: true,
        };

        let result = self
            .client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        let mut stream = result.bytes_stream();

        {
            let mut h = self.history.write().unwrap();
            h.push(Message {
                role: Role::Assistant,
                content: Some(String::new()),
                tool_calls: None,
                tool_call_id: None,
                think_content: Some(String::new()),
            });
        }
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.unwrap();

            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        return Ok(());
                    }

                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            match self.history.write() {
                                Ok(mut h) => {
                                    let len = h.clone().len() - 1;
                                    if let Some(ref mut s) = h[len].content {
                                        s.push_str(content);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                    panic!()
                                }
                            }
                            std::io::Write::flush(&mut std::io::stdout())?;
                        }
                        if let Some(content) =
                            json["choices"][0]["delta"]["reasoning_content"].as_str()
                        {
                            match self.history.write() {
                                Ok(mut h) => {
                                    let len = h.clone().len() - 1;
                                    if let Some(ref mut s) = h[len].think_content {
                                        s.push_str(content);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                    panic!()
                                }
                            }
                            std::io::Write::flush(&mut std::io::stdout())?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
