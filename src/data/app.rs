use crate::data::llm::LLM;

pub struct App {
    pub llm_handle: LLM,
    pub input: String,
    pub is_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            llm_handle: LLM::new(
                std::env::var("API_KEY").unwrap_or_else(|_| "api-key".to_string()),
                "http://127.0.0.1:8080/v1/chat/completions".to_string(),
            ),
            input: String::new(),
            is_quit: false,
        }
    }
}

impl App {}
