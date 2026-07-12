use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TelemetryConfig {
    pub service: ServiceConfig,
    pub logs: Option<LogsConfig>,
    pub traces: Option<TracesConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub environment: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogsConfig {
    pub level: String,
    #[serde(default)]
    pub exporters: Vec<LogExporterConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum LogExporterConfig {
    #[serde(rename = "stdout")]
    Stdout { format: String },
    #[serde(rename = "elasticsearch")]
    Elasticsearch { endpoint: String },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TracesConfig {
    pub sample_rate: f64,
    #[serde(default)]
    pub exporters: Vec<TraceExporterConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum TraceExporterConfig {
    #[serde(rename = "otlp")]
    Otlp { endpoint: String },
}

impl TelemetryConfig {
    pub fn from_yaml_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config: TelemetryConfig = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
