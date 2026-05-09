use rslog::{Logger, LogLevel, ConfigBuilder, OutputFormat};

fn main() {
    println!("=== Testing rslog logging library ===");
    
    // Test with default configuration
    let logger = Logger::get_instance();
    
    logger.info("Application started");
    logger.debug("Debug message");
    logger.warn("Warning message");
    logger.error("Error message");
    logger.critical("Critical error");
    
    println!("=== Logs have been sent to background thread ===");
    
    // Give some time for async writer to flush
    std::thread::sleep(std::time::Duration::from_millis(200));
    
    println!("=== Test completed ===");
}
