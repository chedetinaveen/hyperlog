use std::str::FromStr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry, Layer};
use tracing_subscriber::fmt;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry::KeyValue;

use crate::config::{TelemetryConfig, LogExporterConfig, TraceExporterConfig};

pub fn init(config: &TelemetryConfig) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup global trace provider (OpenTelemetry)
    let mut tracer = None;
    if let Some(traces_cfg) = &config.traces {
        for exporter in &traces_cfg.exporters {
            match exporter {
                TraceExporterConfig::Otlp { endpoint } => {
                    let otlp_exporter = opentelemetry_otlp::new_exporter()
                        .tonic()
                        .with_endpoint(endpoint.clone());
                    
                    let provider = opentelemetry_otlp::new_pipeline()
                        .tracing()
                        .with_exporter(otlp_exporter)
                        .with_trace_config(
                            opentelemetry_sdk::trace::Config::default().with_resource(Resource::new(vec![
                                KeyValue::new("service.name", config.service.name.clone()),
                                KeyValue::new("service.namespace", config.service.environment.clone()),
                            ])),
                        )
                        .install_batch(opentelemetry_sdk::runtime::Tokio)?;
                    
                    tracer = Some(provider);
                    break; // Just taking the first for MVP
                }
            }
        }
    }

    // 2. Setup standard logging layer (tracing-subscriber)
    let mut fmt_layer = None;
    let mut env_filter = EnvFilter::new("info");

    if let Some(logs_cfg) = &config.logs {
        env_filter = EnvFilter::from_str(&logs_cfg.level).unwrap_or_else(|_| EnvFilter::new("info"));
        
        for exporter in &logs_cfg.exporters {
            match exporter {
                LogExporterConfig::Stdout { format } => {
                    let layer = fmt::layer();
                    if format == "json" {
                        fmt_layer = Some(layer.json().boxed());
                    } else {
                        fmt_layer = Some(layer.boxed());
                    }
                }
                LogExporterConfig::Elasticsearch { endpoint: _ } => {
                    // MVP: Elasticsearch integration goes here
                    // e.g., using tracing-elastic or similar
                }
            }
        }
    }

    // Default to basic stdout if no format provided
    let fmt_layer = fmt_layer.unwrap_or_else(|| fmt::layer().boxed());

    let registry = Registry::default().with(env_filter).with(fmt_layer);

    // If tracing is configured, attach the OTLP layer
    if let Some(t) = tracer {
        let telemetry_layer = tracing_opentelemetry::layer().with_tracer(t.tracer("omnilog"));
        registry.with(telemetry_layer).init();
    } else {
        registry.init();
    }

    Ok(())
}
