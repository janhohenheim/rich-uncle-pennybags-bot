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
