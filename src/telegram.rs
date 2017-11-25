pub struct Telegram<'a> {
    token: &'a str,
}

impl<'a> Telegram<'a> {
    pub fn new(token: &'a str) -> Self {
        Telegram { token }
    }
}
