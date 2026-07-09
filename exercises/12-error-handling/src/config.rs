//! Config path validation and port parsing with custom errors.

use std::fmt;
use std::path::{Component, Path};

use thiserror::Error;

/// Errors raised while validating config paths, ports, or credentials.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConfigError {
    /// Path string failed structural validation.
    #[error("invalid config path: {0}")]
    InvalidPath(String),

    /// Required credential was empty or too short.
    #[error("empty or missing credential")]
    EmptyCredential,

    /// Path is syntactically valid but not on the deployment allow-list.
    #[error("config file not found on allow-list: {0}")]
    MissingFile(String),

    /// Port string could not be parsed into a valid `u16`.
    #[error("invalid port: {raw}")]
    InvalidPort { raw: String },
}

/// A validated absolute config file path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigPath {
    path: String,
}

impl ConfigPath {
    /// Borrow the validated path string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.path
    }
}

impl fmt::Display for ConfigPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.path)
    }
}

/// Paths that exist in the simulated deployment (no real filesystem access).
const ALLOWED_CONFIG_FILES: &[&str] = &[
    "/etc/soc/config.toml",
    "/opt/sensor/config.yaml",
    "/var/secure/app.yml",
    "/etc/firewall/rules.toml",
];

/// Validate a config file path (absolute, no traversal, allowed extension).
pub fn validate_config_path(raw: &str) -> Result<ConfigPath, ConfigError> {
    if raw.is_empty() {
        return Err(ConfigError::InvalidPath("path is empty".to_owned()));
    }

    let path = Path::new(raw);
    if !path.is_absolute() {
        return Err(ConfigError::InvalidPath(format!(
            "path must be absolute: {raw}"
        )));
    }

    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(ConfigError::InvalidPath(format!(
            "path must not contain '..': {raw}"
        )));
    }

    let ext_ok = path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| matches!(e, "toml" | "yaml" | "yml"));

    if !ext_ok {
        return Err(ConfigError::InvalidPath(format!(
            "unsupported extension (want .toml/.yaml/.yml): {raw}"
        )));
    }

    Ok(ConfigPath {
        path: raw.to_owned(),
    })
}

/// Ensure the path appears on the hardcoded deployment allow-list.
pub fn ensure_exists(path: &ConfigPath) -> Result<(), ConfigError> {
    if ALLOWED_CONFIG_FILES.contains(&path.as_str()) {
        Ok(())
    } else {
        Err(ConfigError::MissingFile(path.as_str().to_owned()))
    }
}

/// Parse a service port in the range 1..=65535.
pub fn parse_port(raw: &str) -> Result<u16, ConfigError> {
    let trimmed = raw.trim();
    let port: u16 = trimmed.parse().map_err(|_| ConfigError::InvalidPort {
        raw: raw.to_owned(),
    })?;
    if port == 0 {
        return Err(ConfigError::InvalidPort {
            raw: raw.to_owned(),
        });
    }
    Ok(port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_accepts_good_path() {
        let path = validate_config_path("/etc/soc/config.toml");
        assert!(path.is_ok());
    }

    #[test]
    fn validate_rejects_bad_paths() {
        assert!(matches!(
            validate_config_path(""),
            Err(ConfigError::InvalidPath(_))
        ));
        assert!(matches!(
            validate_config_path("relative.toml"),
            Err(ConfigError::InvalidPath(_))
        ));
        assert!(matches!(
            validate_config_path("/etc/../secret.yaml"),
            Err(ConfigError::InvalidPath(_))
        ));
    }

    #[test]
    fn ensure_exists_checks_allow_list() {
        let ok = validate_config_path("/etc/soc/config.toml");
        if let Ok(path) = ok {
            assert!(ensure_exists(&path).is_ok());
        }
        let missing = validate_config_path("/etc/unknown/config.toml");
        if let Ok(path) = missing {
            assert!(matches!(
                ensure_exists(&path),
                Err(ConfigError::MissingFile(_))
            ));
        }
    }

    #[test]
    fn parse_port_validates_range() {
        assert_eq!(parse_port("443"), Ok(443));
        assert!(matches!(
            parse_port("0"),
            Err(ConfigError::InvalidPort { .. })
        ));
        assert!(matches!(
            parse_port("not-a-port"),
            Err(ConfigError::InvalidPort { .. })
        ));
    }
}
