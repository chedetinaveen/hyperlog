# OmniLog

**Universal Telemetry & Logging Engine for Python, powered by Rust.**

OmniLog is a blazingly fast, zero-boilerplate telemetry engine. It automatically intercepts your standard Python logs and streams them into OpenTelemetry, Elasticsearch, or Prometheus using a highly optimized native Rust background engine.

## Why OmniLog? (vs. The Competition)

The telemetry space is incredibly crowded with major players like **OpenTelemetry**, **Datadog**, and **Vector**, but OmniLog fundamentally solves the biggest pain points they haven't addressed.

### 1. vs. Standard OpenTelemetry SDKs (e.g., `opentelemetry-python`)
**The Competitor:** The official Python OpenTelemetry SDK is written in pure Python. When your app generates thousands of logs or traces, the SDK formats them, batches them, and serializes them (usually to Protobuf) all while holding the Python **Global Interpreter Lock (GIL)**. This massively degrades the performance of high-throughput apps because your business logic has to pause while telemetry is serialized.

**The OmniLog Difference:** OmniLog is a **Rust native extension**. When you call `logging.info()`, OmniLog instantly passes a pointer to the raw data into Rust and returns control to Python immediately. Rust does all the formatting, Protobuf serialization, and gRPC network exporting on a background thread entirely outside of the GIL. **It guarantees near-zero performance degradation.**

### 2. vs. Sidecar Agents (e.g., Datadog Agent, Vector, FluentBit)
**The Competitor:** To avoid the performance hit of doing telemetry inside Python, tools like FluentBit or the Datadog Agent require you to run a completely separate process (a sidecar) on your machine. Your Python app stringifies its logs to JSON and writes them to standard output (`stdout`). The sidecar tails `stdout`, parses the JSON, and ships it. 

**The OmniLog Difference:** Writing to `stdout` and parsing JSON stringified logs is computationally expensive and strips away rich, structured in-memory data (like active Trace IDs). Because OmniLog lives *inside* the Python application's memory space, there is no inter-process communication (IPC) or JSON serialization overhead, and you don't have to manage deploying a separate sidecar container in Kubernetes.

### 3. The "Boilerplate" Problem
**The Competitor:** Setting up standard OpenTelemetry in Python requires installing 5 to 10 different pip packages (`opentelemetry-api`, `opentelemetry-sdk`, `opentelemetry-exporter-otlp`, `opentelemetry-instrumentation-logging`) and writing 50 lines of complex initialization code.

**The OmniLog Difference:** A single `pip install hyperlog`, a single `telemetry.yaml` file, and one line of code: `hyperlog_auto.auto_instrument("telemetry.yaml")`.

---

## Quick Start

### 1. Install
```bash
pip install hyperlog
```

### 2. Configure (`telemetry.yaml`)
```yaml
service:
  name: "my-python-service"
  environment: "production"
logs:
  level: "info"
  exporters:
    - type: "stdout"
      format: "json"
traces:
  sample_rate: 1.0
  exporters:
    - type: "otlp"
      endpoint: "http://localhost:4317"
```

### 3. Auto-Instrument
Just call `auto_instrument` once at the start of your application. All standard Python logging will instantly bypass the GIL and route through Rust!

```python
import logging
from hyperlog import auto_instrument

# Hijack standard Python logging
auto_instrument("telemetry.yaml")

# Use standard logging as normal!
logging.info("This is processed in Rust at native speed!")
logging.error("So is this!")
```
