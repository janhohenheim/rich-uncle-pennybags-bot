use reqwest;
use telegram::error::*;
use telegram::model::*;

pub struct Api {
    api_url: String,
    username: String,
    client: reqwest::Client,
}
impl Api {
    pub fn new(token: &str, username: &str) -> Self {
        const API_URL_PREFIX: &str = "https://api.telegram.org/bot";
        Api {
            api_url: format!("{}{}/", API_URL_PREFIX, token),
            username: username.to_string(),
            client: reqwest::Client::new(),
        }
    }
    pub fn send_message(&self, chat_id: i64, text: &str) -> Result<Response<Message>> {
        Ok(self.make_request("sendMessage")
            .form(&[
                ["chat_id", &chat_id.to_string()],
                ["text", text],
                ["disable_notification", "true"],
                ["parse_mode", "Markdown"],
            ])
            .send()?
            .json()?)
    }

    pub fn extract_text(&self, message: &Message) -> Option<String> {
        match message.text {
            Some(ref text) => {
                let mention = format!("@{}", self.username);
                Some(text.trim_right_matches(&mention).to_string())
            }
            None => None,
        }
    }

    fn make_request(&self, method: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.api_url, method);
        self.client.get(&url)
    }
}
