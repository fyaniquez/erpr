//! src/configuration.rs
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_derive::Deserialize;

/*
 * author: fyaniquez
 * date: 30/09/2022
 * purpose: lee/procesa parametros de configuración 
 * directorio / archivo
 * configuration/base.yaml: parametros comunes
 * configuration/local.yaml: parametros para pruebas
 * configuration/production.yaml: parametros para producción
 * APP_ENVIRONMENT=local/production establece cual es el ambiente
 */

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path 
        = std::env::current_dir().expect("Directorio actual indefinido");
    let conf_directory = base_path.join("configuration");
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("No esta declarada la variable APP_ENVIRONMENT");
    let environment_file = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(conf_directory.join("base.yaml")))
        .add_source(config::File::from(conf_directory.join(&environment_file)))
        .build()?;
    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    /// convierte la enumeración Environment a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    /// convierte un string a la enumeración Environment
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} Error, use `local` o `production`.",
                other
            )),
        }
    }
}
