#[derive(Deserialize)]
pub struct Update {
    pub update_id: i32,
    pub message: Message,
}

#[derive(Deserialize)]
pub struct Message {
    pub message_id: i32,
    pub chat: Chat,
    pub text: String,
}

#[derive(Deserialize)]
pub struct Chat {
    pub id: i32,
}


#[derive(Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub error_code: Option<i32>,
    pub description: Option<String>,
    pub result: T,
}
