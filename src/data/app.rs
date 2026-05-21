use ratatui::widgets::ListState;

use crate::data::llm::LLM;

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Input,
    CurrChat,
    ScrollChat,
    Code,
}

pub struct App {
    pub input_bar_size: u16,
    pub mode: Mode,
    pub llm_handle: LLM,
    pub input: String,
    pub exit: bool,
    pub cursor: bool,
    pub scroll: u16,
    pub histoy_chat_list_state: ListState,
    pub l_histoy_chat_list_state: ListState,
}

pub struct Code {
    pub language: String,
    pub code: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input_bar_size: 3,
            mode: Mode::Normal,
            llm_handle: LLM::new(
                std::env::var("API_KEY").unwrap_or_else(|_| "your-api-key".to_string()),
                "http://127.0.0.1:8080/v1/chat/completions".to_string(),
            ),
            input: String::new(),
            exit: false,
            cursor: false,
            scroll: 0,
            histoy_chat_list_state: ListState::default(),
            l_histoy_chat_list_state: ListState::default(),
        }
    }
}

impl App {
    pub fn next_current_chat_select(&mut self) {
        let i = match self.histoy_chat_list_state.selected() {
            Some(i) => {
                if i >= self.llm_handle.history.read().unwrap().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.histoy_chat_list_state.select(Some(i));
    }
    pub fn prew_current_chat_select(&mut self) {
        let i = match self.histoy_chat_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.llm_handle.history.read().unwrap().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.histoy_chat_list_state.select(Some(i));
    }

    pub fn scroll_k(&mut self) {
        self.scroll += 1;
    }

    pub fn scroll_j(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }
}
