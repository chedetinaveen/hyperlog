import omnilog
import time

def main():
    print("Initializing OmniLog native extension...")
    omnilog.init_telemetry("telemetry.yaml")
    
    print("Emitting logs via native extension...")
    omnilog.log_info("This is an info log from Python, serialized natively in Rust!")
    omnilog.log_error("This is an error log!")
    
    print("Sleeping for 1 second to flush traces...")
    time.sleep(1)
    
if __name__ == "__main__":
    main()
