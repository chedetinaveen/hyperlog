from hyperlog._hyperlog import init_telemetry, log_info, log_error, log_record
from hyperlog.auto import auto_instrument, OmniLogHandler

__all__ = [
    "init_telemetry",
    "log_info",
    "log_error",
    "log_record",
    "auto_instrument",
    "OmniLogHandler",
]
