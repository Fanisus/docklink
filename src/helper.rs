#[derive(Debug)]
pub struct Docker {
    host: String,
    port: u16,
    username: String,
    password: String,
    protocol: String,
}
#[derive(Default)]
pub struct DockerBuilder {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    protocol: Option<String>,
}
impl Docker {
    fn new() -> DockerBuilder {
        DockerBuilder {
            host: Some("127.0.0.1".to_string()),
            port: Some(2375),
            username: Some("".to_string()),
            password: Some("".to_string()),
            protocol: Some("http".to_string()),
        }
    }
}
impl DockerBuilder {
    pub fn host(&mut self, host: String) -> &mut Self {
        self.host = Some(host.into());
        self
    }
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }
    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username.into());
        self
    }
    pub fn password(&mut self, password: String) -> &mut Self {
        self.password = Some(password);
        self
    }
    pub fn build(&mut self) {}
}