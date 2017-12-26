pub mod toml;

#[derive(Debug, Clone)]
pub struct Coin {
    pub short_name: String,
    pub name: toml::Name,
}
