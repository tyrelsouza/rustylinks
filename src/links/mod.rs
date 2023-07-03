use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RustyLinks {
    config: Config,
    links: Vec<Links>,
    pub metadata: Option<MetaData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    title: String,
    name: String,
    description: String,
    avatar: String,
    background: String,
    background_opacity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    title: String,
    links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    text: String,
    icon: String,
    href: Option<String>,
    copy: Option<String>,
}
