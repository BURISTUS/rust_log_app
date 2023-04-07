use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    pub application: ApplicationConfig,
    pub connection: ConnectionConfig,
    // pub file: FileConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationConfig {
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ConnectionConfig {
    pub min_request_duration: u64,
    pub max_request_duration: u64,
    pub timeout: u64,
    pub max_requests: u8,
}

// #[derive(Deserialize, Clone, Debug)]
// pub struct FileConfig {
//     pub file_path: String,
// }

pub fn get_configuration() -> Result<ApplicationSettings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}
