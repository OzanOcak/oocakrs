use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
    pub allowed_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let port = env_or("PORT", "3000")
            .parse()
            .context("PORT must be a number")?;
        let db_port = env_or("DB_PORT", "5432")
            .parse()
            .context("DB_PORT must be a number")?;

        let allowed_origins = env_or(
            "ALLOWED_ORIGINS",
            "http://localhost:5173",
        )
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

        Ok(Self {
            port,
            db_host: env_or("DB_HOST", "localhost"),
            db_port,
            db_username: env_or("DB_USERNAME", "postgres"),
            db_password: env_or("DB_PASSWORD", "postgres"),
            db_name: env_or("DB_NAME", "database"),
            allowed_origins,
        })
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_username, self.db_password, self.db_host, self.db_port, self.db_name
        )
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}