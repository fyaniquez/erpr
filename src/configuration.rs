//! src/configuration.rs
//! author: fyaniquez
//! date: 13/10/2022
//! purpose: parametros para personalizar los varios componentes de la app
//! configuration/base.yaml: parametros comunes
//! configuration/local.yaml: parametros para pruebas
//! configuration/production.yaml: parametros para producción
//! APP_ENVIRONMENT=local/production establece cual es el ambiente

use std::env::current_dir;

/// Reunión de parametros
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}


/// Parámetros de la base de datos
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub user: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

/// Parametros del servidor de aplicaciones
#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

impl ApplicationSettings {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
/// Lee las configuraciones de los archivos configuration/*.yaml
/// selecciona la configuración en función a APP_ENVIRONMENT
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = current_dir().expect("Error en directorio actual");
    let conf_dir = base_path.join("configuration");
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("No esta declarada la variable APP_ENVIRONMENT");
    let environment_file = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(conf_dir.join("base.yaml")))
        .add_source(config::File::from(conf_dir.join(&environment_file)))
        .build()?;
    settings.try_deserialize::<Settings>()
}

/// describe los dos modos de configuración de la aplicacion
/// local = desarrollo
/// production = producción
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

/// convierte un string al enum Environment
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
