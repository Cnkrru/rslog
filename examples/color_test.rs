use rslog::{Logger, LogLevel, ConfigBuilder, OutputFormat, Color, LogColorScheme};

fn main() {
    println!("=== Testing rslog with colors ===");
    
    // Test 1: Default configuration with colors
    println!("\n--- Test 1: Default colors ---");
    let logger = Logger::get_instance();
    
    logger.debug("Debug message with default colors");
    logger.info("Info message with default colors");
    logger.warn("Warning message with default colors");
    logger.error("Error message with default colors");
    logger.critical("Critical message with default colors");
    
    // Give some time for async writer to flush
    std::thread::sleep(std::time::Duration::from_millis(200));
    
    // Test 2: Custom color scheme
    println!("\n--- Test 2: Custom color scheme ---");
    let custom_scheme = LogColorScheme::new(
        Color::BrightBlack,    // Trace
        Color::BrightCyan,    // Debug
        Color::BrightGreen,   // Info
        Color::BrightYellow,  // Warn
        Color::BrightRed,     // Error
        Color::Magenta,       // Critical
    );
    
    let config = ConfigBuilder::new()
        .color_scheme(custom_scheme)
        .console_colors(true)
        .build();
    
    Logger::init_with_config(config);
    let logger = Logger::get_instance();
    
    logger.debug("Debug with custom colors");
    logger.info("Info with custom colors");
    logger.warn("Warn with custom colors");
    logger.error("Error with custom colors");
    logger.critical("Critical with custom colors");
    
    std::thread::sleep(std::time::Duration::from_millis(200));
    
    // Test 3: Disable colors
    println!("\n--- Test 3: Colors disabled ---");
    let config = ConfigBuilder::new()
        .console_colors(false)
        .build();
    
    Logger::init_with_config(config);
    let logger = Logger::get_instance();
    
    logger.debug("Debug without colors");
    logger.info("Info without colors");
    logger.warn("Warn without colors");
    logger.error("Error without colors");
    logger.critical("Critical without colors");
    
    std::thread::sleep(std::time::Duration::from_millis(200));
    
    // Test 4: Dynamic color toggling
    println!("\n--- Test 4: Dynamic color toggling ---");
    let config = ConfigBuilder::new()
        .console_colors(true)
        .build();
    
    Logger::init_with_config(config);
    let logger = Logger::get_instance();
    
    logger.info("Colors enabled");
    
    // Disable colors at runtime
    logger.set_console_colors(false);
    logger.info("Colors disabled at runtime");
    
    // Re-enable colors
    logger.set_console_colors(true);
    logger.info("Colors re-enabled at runtime");
    
    std::thread::sleep(std::time::Duration::from_millis(200));
    
    // Test 5: Test all ANSI colors
    println!("\n--- Test 5: Testing all ANSI colors ---");
    let colors = [
        (Color::Black, "Black"),
        (Color::Red, "Red"),
        (Color::Green, "Green"),
        (Color::Yellow, "Yellow"),
        (Color::Blue, "Blue"),
        (Color::Magenta, "Magenta"),
        (Color::Cyan, "Cyan"),
        (Color::White, "White"),
        (Color::BrightBlack, "BrightBlack"),
        (Color::BrightRed, "BrightRed"),
        (Color::BrightGreen, "BrightGreen"),
        (Color::BrightYellow, "BrightYellow"),
        (Color::BrightBlue, "BrightBlue"),
        (Color::BrightMagenta, "BrightMagenta"),
        (Color::BrightCyan, "BrightCyan"),
        (Color::BrightWhite, "BrightWhite"),
    ];
    
    for (color, name) in colors.iter() {
        let colored = Color::colorize(name, *color);
        println!("{}", colored);
    }
    
    println!("\n=== Color test completed ===");
}