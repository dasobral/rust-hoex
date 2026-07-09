//! Integration tests for the `exercise_variables` crate
//!
//! These tests exercise the public library API end-to-end:
//! orchestration helpers, cross-module utilities, and each physics exercise.

use variables_exercises::conversions::{
    celsius_to_fahrenheit, celsius_to_kelvin, hydrogen_energy_level,
};
use variables_exercises::display::{format_scientific, format_vector_3d};
use variables_exercises::electromagnetic::calculate_vector_magnitude;
use variables_exercises::quantum::{approximate_energy_ev, validate_quantum_numbers};
use variables_exercises::temperature::convert_celsius_all;
use variables_exercises::{get_exercise_list, run_all, run_exercise};

#[test]
fn test_exercise_list_contains_all_three() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 3);

    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"quantum"));
    assert!(names.contains(&"electromagnetic"));
    assert!(names.contains(&"temperature"));

    for exercise in &list {
        assert!(!exercise.description.is_empty());
        assert!(!exercise.concepts.is_empty());
    }
}

#[test]
fn test_run_each_exercise() {
    assert!(run_exercise("quantum", false).is_ok());
    assert!(run_exercise("electromagnetic", false).is_ok());
    assert!(run_exercise("temperature", false).is_ok());
}

#[test]
fn test_run_unknown_exercise_errors() {
    let err = run_exercise("nuclear", false);
    assert!(err.is_err());
    if let Err(e) = err {
        let message = format!("{e}");
        assert!(message.contains("Unknown exercise"));
    }
}

#[test]
fn test_run_all() {
    assert!(run_all(false).is_ok());
}

#[test]
fn test_cross_module_quantum_and_display() {
    let energy = hydrogen_energy_level(2, 1);
    assert!(energy.is_ok());
    if let Ok(e) = energy {
        let formatted = format_scientific(e, 3);
        assert!(formatted.contains('e') || formatted.contains('E'));
    }

    assert!(approximate_energy_ev(1).is_ok_and(|e| e == -13));
    assert!(validate_quantum_numbers(3, 2, -2).is_ok());
    assert!(validate_quantum_numbers(3, 2, 3).is_err());
}

#[test]
fn test_cross_module_electromagnetic() {
    let mag = calculate_vector_magnitude(100, -50, 0);
    assert!(mag.is_ok_and(|m| m > 111.0 && m < 112.0));

    let formatted = format_vector_3d(100, -50, 0);
    assert_eq!(formatted, "(100, -50, 0) V/m");
}

#[test]
#[allow(clippy::float_cmp)]
fn test_cross_module_temperature() {
    let converted = convert_celsius_all(0);
    assert!(converted.is_ok());
    if let Ok((f, k)) = converted {
        assert!((f - 32.0).abs() < f32::EPSILON);
        assert_eq!(k, 273);
    }

    assert!((celsius_to_fahrenheit(100) - 212.0).abs() < f32::EPSILON);
    assert!(celsius_to_kelvin(-10).is_ok_and(|k| k == 263));
    assert!(celsius_to_kelvin(-300).is_err());
}
