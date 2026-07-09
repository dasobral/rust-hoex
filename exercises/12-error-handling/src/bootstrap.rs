//! Secure bootstrap pipeline combining path and credential validation.

use anyhow::Result;

use crate::config::{ConfigError, ConfigPath, ensure_exists, parse_port, validate_config_path};
use crate::credentials::{
    mask_token, optional_credential, parse_optional_token, require_credential,
};

/// Validate path and require a credential in one pipeline.
pub fn secure_bootstrap(path: &str, secret: &str) -> Result<(ConfigPath, String), ConfigError> {
    let config = validate_config_path(path)?;
    ensure_exists(&config)?;
    let cred = require_credential(secret)?;
    Ok((config, cred))
}

/// Load settings: validate path, ensure it exists, optionally attach a token.
pub fn load_settings(
    path: &str,
    token_raw: &str,
) -> Result<(ConfigPath, Option<String>), ConfigError> {
    let config = validate_config_path(path)?;
    ensure_exists(&config)?;
    let token = parse_optional_token(token_raw).map(|t| mask_token(&t));
    Ok((config, token))
}

/// Parse port and optional credential together (demonstrates `?` chaining).
pub fn bind_service(
    port_raw: &str,
    token: Option<&str>,
) -> Result<(u16, Option<String>), ConfigError> {
    let port = parse_port(port_raw)?;
    let cred = optional_credential(token)?;
    Ok((port, cred))
}

/// Run the bootstrap exercise.
pub fn run(verbose: bool) -> Result<()> {
    println!("🔐 Bootstrap — secure startup pipeline");
    println!();

    match secure_bootstrap("/etc/soc/config.toml", "hunter2-secret") {
        Ok((path, cred)) => {
            println!("  config: {path}");
            println!("  credential accepted (len={})", cred.len());
        }
        Err(e) => println!("  bootstrap failed: {e}"),
    }

    if let Err(e) = secure_bootstrap("/etc/unknown/config.toml", "hunter2-secret") {
        println!("  missing file: {e}");
    }

    if let Err(e) = secure_bootstrap("/etc/soc/config.toml", "short") {
        println!("  bad credential: {e}");
    }

    match bind_service("8443", Some("service-token-99")) {
        Ok((port, cred)) => {
            println!("  bind port {port}, cred present: {}", cred.is_some());
        }
        Err(e) => println!("  bind failed: {e}"),
    }

    if verbose {
        println!();
        println!("  load_settings masks optional tokens for safe logs:");
        if let Ok((path, token)) = load_settings("/opt/sensor/config.yaml", "super-secret-key") {
            println!("    path={path}, token={token:?}");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secure_bootstrap_ok() {
        let result = secure_bootstrap("/etc/soc/config.toml", "password1");
        assert!(result.is_ok());
        if let Ok((path, cred)) = result {
            assert_eq!(path.as_str(), "/etc/soc/config.toml");
            assert_eq!(cred, "password1");
        }
    }

    #[test]
    fn secure_bootstrap_rejects_missing_and_short() {
        assert!(secure_bootstrap("/etc/missing/config.toml", "password1").is_err());
        assert!(secure_bootstrap("/etc/soc/config.toml", "x").is_err());
    }

    #[test]
    fn bind_service_chains_results() {
        let result = bind_service("443", None);
        assert!(result.is_ok());
        if let Ok((port, cred)) = result {
            assert_eq!(port, 443);
            assert!(cred.is_none());
        }
    }
}
