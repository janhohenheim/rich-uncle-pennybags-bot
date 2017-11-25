use super::super::reqwest;
use telegram::error::*;
use telegram::types::*;

pub struct Api {
    api_url: String,
    client: reqwest::Client,
}
impl Api {
    pub fn new(token: &str) -> Self {
        const API_URL_PREFIX: &str = "https://api.telegram.org/bot";
        Api {
            api_url: format!("{}{}/", API_URL_PREFIX, token),
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
    fn make_request(&self, method: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.api_url, method);
        self.client.get(&url)
    }
}
