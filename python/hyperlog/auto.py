import logging
import hyperlog._hyperlog as hyperlog_native

class OmniLogHandler(logging.Handler):
    """
    A Python logging handler that intercepts all standard Python logs
    and pipes them securely into the OmniLog Rust engine via PyO3.
    """
    def emit(self, record):
        try:
            # Format the message (this resolves %s formatting)
            msg = self.format(record)
            
            # Send the record level (10, 20, 30, etc.) and the message to Rust
            hyperlog_native.log_record(record.levelno, msg)
        except Exception:
            self.handleError(record)

def auto_instrument(config_path: str):
    """
    Initializes the native telemetry engine and hijacks Python's root logger.
    """
    # 1. Initialize Rust Engine
    hyperlog_native.init_telemetry(config_path)
    
    # 2. Hijack the Python logging system
    root_logger = logging.getLogger()
    
    # Remove existing handlers to prevent duplicate local logging
    for handler in root_logger.handlers[:]:
        root_logger.removeHandler(handler)
        
    # Attach our native Rust bridge handler
    hyperlog_handler = OmniLogHandler()
    hyperlog_handler.setFormatter(logging.Formatter('%(message)s'))
    root_logger.addHandler(hyperlog_handler)
    
    # Set the root logger to capture all levels; the Rust engine will filter based on telemetry.yaml
    root_logger.setLevel(logging.DEBUG)
