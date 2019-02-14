#[derive(Clone, Deserialize)]
pub struct Target {
    pub name: String,
    pub shared_secret: String,
    pub index_key: String,
    pub encrypt_key: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub target: Vec<Target>,
}
