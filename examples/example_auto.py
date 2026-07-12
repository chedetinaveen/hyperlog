import logging
from hyperlog_auto import auto_instrument
import time

def main():
    print("Auto-instrumenting Python logging...")
    auto_instrument("telemetry.yaml")
    
    print("Emitting standard Python logs (which will now route through Rust!)...")
    logging.debug("This debug log might be filtered out by Rust!")
    logging.info("This is an INFO log from Python's standard logging module.")
    logging.warning("This is a WARNING log.")
    logging.error("This is an ERROR log.")
    logging.critical("This is a CRITICAL log.")
    
    print("Sleeping for 1 second to flush traces...")
    time.sleep(1)
    
if __name__ == "__main__":
    main()
