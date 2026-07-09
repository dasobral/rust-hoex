//! Integration tests for `example_errorhandling`.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use example_errorhandling::{
    ConfigError, load_settings, optional_credential, parse_optional_token, secure_bootstrap,
    validate_config_path,
};

#[test]
fn valid_path_and_settings() {
    let path = validate_config_path("/var/app/settings.yaml").expect("path ok");
    assert_eq!(path.as_str(), "/var/app/settings.yaml");

    let (p, tok) = load_settings("/var/app/settings.yaml", "abcdefghij").expect("load");
    assert_eq!(p.as_str(), "/var/app/settings.yaml");
    assert!(tok.is_some());
}

#[test]
fn errors_are_typed() {
    assert_eq!(validate_config_path(""), Err(ConfigError::EmptyPath));
    assert!(matches!(
        validate_config_path("x.toml"),
        Err(ConfigError::NotAbsolute(_))
    ));
}

#[test]
fn option_and_result_pipeline() {
    assert!(parse_optional_token("").is_none());
    assert!(optional_credential(None).unwrap().is_none());
    assert!(secure_bootstrap("/a.toml", "password1").is_ok());
    assert!(secure_bootstrap("/a.toml", "no").is_err());
}
