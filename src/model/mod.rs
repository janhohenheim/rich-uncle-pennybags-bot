pub mod toml;

#[derive(Debug, Clone)]
pub struct Coin {
    pub short_name: String,
    pub name: toml::Name,
}

impl Coin {
    pub fn long_name(&self) -> &str {
        match self.name {
            toml::Name::Simple(ref long_name) => &long_name,
            toml::Name::Detailed(ref detailed_name) => &detailed_name.long_name,
        }
    }
}
