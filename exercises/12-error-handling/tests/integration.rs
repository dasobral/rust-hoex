//! Integration tests for `exercise_errorhandling`.

use errorhandling_exercises::{
    ConfigError, bind_service, ensure_exists, get_exercise_list, optional_credential,
    parse_optional_token, parse_port, require_credential, run_all, run_exercise, secure_bootstrap,
    validate_config_path,
};

#[test]
fn exercise_list_has_three_entries() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 3);
}

#[test]
fn run_each_exercise() {
    assert!(run_exercise("paths", false).is_ok());
    assert!(run_exercise("credentials", false).is_ok());
    assert!(run_exercise("bootstrap", false).is_ok());
}

#[test]
fn run_all_succeeds() {
    assert!(run_all(false).is_ok());
}

#[test]
fn validate_and_ensure_path() {
    let path = validate_config_path("/etc/soc/config.toml");
    assert!(path.is_ok());
    if let Ok(config) = path {
        assert!(ensure_exists(&config).is_ok());
    }
}

#[test]
fn missing_file_on_allow_list() {
    let path = validate_config_path("/etc/not-deployed/config.toml");
    assert!(path.is_ok());
    if let Ok(config) = path {
        assert!(matches!(
            ensure_exists(&config),
            Err(ConfigError::MissingFile(_))
        ));
    }
}

#[test]
fn parse_port_and_credentials() {
    assert_eq!(parse_port("443"), Ok(443));
    assert_eq!(parse_optional_token(""), None);
    assert!(require_credential("short").is_err());
    assert_eq!(optional_credential(None).ok(), Some(None));
}

#[test]
fn secure_bootstrap_pipeline() {
    let ok = secure_bootstrap("/opt/sensor/config.yaml", "valid-secret");
    assert!(ok.is_ok());
    assert!(secure_bootstrap("/opt/sensor/config.yaml", "bad").is_err());
}

#[test]
fn bind_service_integration() {
    let result = bind_service("8080", Some("long-token-1"));
    assert!(result.is_ok());
    if let Ok((port, cred)) = result {
        assert_eq!(port, 8080);
        assert!(cred.is_some());
    }
}
