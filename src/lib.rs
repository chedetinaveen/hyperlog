pub mod config;
pub mod telemetry;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use tracing::{info, error, debug};

/// Initialize the telemetry engine from a YAML configuration file.
#[pyfunction]
fn init_telemetry(config_path: String) -> PyResult<()> {
    let config = config::TelemetryConfig::from_yaml_file(&config_path)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse config: {}", e)))?;
    
    telemetry::init(&config)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to init telemetry: {}", e)))?;
    
    Ok(())
}

/// Emit an info log
#[pyfunction]
fn log_info(message: String) {
    info!("{}", message);
}

/// Emit an error log
#[pyfunction]
fn log_error(message: String) {
    error!("{}", message);
}

/// Emit a generic log record
#[pyfunction]
fn log_record(level: u8, message: String) {
    // Standard Python logging levels:
    // CRITICAL = 50, ERROR = 40, WARNING = 30, INFO = 20, DEBUG = 10
    match level {
        10 => debug!("{}", message),
        20 => info!("{}", message),
        30 => tracing::warn!("{}", message),
        40 => error!("{}", message),
        50 => error!("CRITICAL: {}", message),
        _ => info!("{}", message),
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn omnilog(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_telemetry, m)?)?;
    m.add_function(wrap_pyfunction!(log_info, m)?)?;
    m.add_function(wrap_pyfunction!(log_error, m)?)?;
    m.add_function(wrap_pyfunction!(log_record, m)?)?;
    Ok(())
}
