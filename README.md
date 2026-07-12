# HyperLog 🚀

**HyperLog** is a zero-copy, native Rust telemetry engine for Python applications. 
It replaces the massive overhead of standard OpenTelemetry SDKs and sidecar agents (like FluentBit) by embedding a blazingly fast Rust engine directly inside your Python process.

## Why HyperLog?
* **Zero GIL Blocking**: All Protobuf serialization and gRPC networking happens on a Rust background thread. Python never waits.
* **No Sidecars Needed**: Because it lives in your app's memory, there is no JSON stdout parsing overhead.
* **Zero Boilerplate**: Instrument your app with a single `hyperlog_auto.py` import and a `telemetry.yaml`.

## Installation
```bash
pip install hyperlog
```

## Quick Start (Auto-Instrumentation)
With Auto-Instrumentation, HyperLog automatically intercepts Python's standard `logging` module.

1. **Create `telemetry.yaml`**
```yaml
service:
  name: "my-python-app"
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

2. **Initialize in Python**
```python
import logging
from hyperlog import auto_instrument

# Hijack standard Python logging
auto_instrument("telemetry.yaml")

# Use standard logging as normal!
logging.info("This is processed in Rust at native speed!")
logging.error("So is this!")
```

## Documentation
For deep-dive examples and API reference, visit the [HyperLog GitHub Pages](https://chedetinaveen.github.io/hyperlog/).
