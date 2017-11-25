#[derive(Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub channel_post: Option<Message>,
}

#[derive(Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub chat: Chat,
    pub text: Option<String>,
}

#[derive(Deserialize)]
pub struct Chat {
    pub id: i64,
}


#[derive(Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub error_code: Option<i32>,
    pub description: Option<String>,
    pub result: T,
}
