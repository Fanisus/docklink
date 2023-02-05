
#[derive(Debug)]
pub struct Docker {
    host: String,
    port: u16,
    protocol: String,         // should be enum
    password: Option<String>, // (name, value)
    username: Option<String>,
}

#[derive(Clone)]
pub struct DockerBuilder {
    host: Option<String>,
    port: Option<u16>,
    protocol: Option<String>,
    password: Option<String>,
    username: Option<String>,
}

impl DockerBuilder {
    pub fn new() -> Self {
        // The Default Configurations
        DockerBuilder {
            host: Some("127.0.0.1".to_string()),
            port: Some(2375),
            protocol: Some("http".to_string()),
            password: None,
            username: None,
        }
    }

    pub fn host(&mut self, host: impl Into<String>) -> &mut Self {
        self.host.insert(host.into());
        self
    }
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }
    pub fn protocol(&mut self, protocol: impl Into<String>) -> &mut Self {
        self.protocol.insert(protocol.into());
        self
    }
    pub fn username(&mut self, username: impl Into<String>) -> &mut Self {
        self.username.insert(username.into());
        self
    }
    pub fn password(&mut self, password: impl Into<String>) -> &mut Self {
        self.password.insert(password.into());
        self
    }

    pub fn build(&self) -> Result<Docker, String> {
        let Some(host) = self.host.as_ref() else {
			return Err("No Host".to_string());
		};
        let protocol = self.protocol.clone().unwrap_or_else(|| "GET".to_string());

        Ok(Docker {
            host: host.to_string(),
            port: self.port.unwrap_or(2375),
            protocol,
            password: self.password.clone(),
            username: self.username.clone(),
        })
    }
}
