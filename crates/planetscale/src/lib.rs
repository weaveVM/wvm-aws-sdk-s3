use planetscale_driver::PSConnection;

#[derive(Debug, Default, Clone)]
pub struct PlanetScaleDriver {
    pub host: String,
    pub username: String,
    pub password: String,
}

impl PlanetScaleDriver {
    pub fn new(host: String, username: String, password: String) -> Self {
        Self {
            host,
            username,
            password,
        }
    }

    pub fn get_conn(&self) -> PSConnection {
        PSConnection::new(&self.host, &self.username, &self.password)
    }
}
