use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct AppSettings {
    pub settings: SettingsConfig 
}


#[derive(Deserialize, Clone, Debug)]
pub struct SettingsConfig {
    pub timeout: u64
}


pub fn get_configuration() -> Result<AppSettings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}
