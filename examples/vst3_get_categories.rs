//! Simple VST3 audio processing example
//!
//! This example demonstrates:
//! - Loading a VST3 plugin
//! - Initializing it for audio processing
//! - Processing audio through the plugin
//! - Accessing parameters

#[cfg(all(
    not(target_os = "ios"),
    not(target_os = "tvos"),
    not(target_os = "watchos"),
    not(target_os = "visionos")
))]
use rack::vst3::Vst3Scanner;
#[cfg(all(
    not(target_os = "ios"),
    not(target_os = "tvos"),
    not(target_os = "watchos"),
    not(target_os = "visionos")
))]
use rack::{PluginInstance, PluginScanner, Result};

#[cfg(all(
    not(target_os = "ios"),
    not(target_os = "tvos"),
    not(target_os = "watchos"),
    not(target_os = "visionos")
))]
fn main() -> Result<()> {
    println!("VST3 Audio Processing Example\n");

    // Scan for VST3 plugins
    let scanner = Vst3Scanner::new()?;
    let plugins = scanner.scan()?;

    if plugins.is_empty() {
        println!("No VST3 plugins found.");
        return Ok(());
    }

    // Find an effect plugin (not an instrument)
    plugins.iter().for_each(|p| {
        println!("Found plugin: {}", p.name);
    });
    let plugin_info = plugins
        .iter()
        .find(|p| p.plugin_type == rack::PluginType::Instrument)
        .or_else(|| plugins.first())
        .unwrap();

    println!("Loading plugin: {}", plugin_info.name);
    println!("Manufacturer: {}", plugin_info.manufacturer);
    println!();

    // Load the plugin
    let mut plugin = scanner.load(plugin_info)?;

    // Initialize for 48kHz, 512 samples per buffer
    let sample_rate = 48000.0;
    let buffer_size = 512;

    println!("Initializing plugin...");
    plugin.initialize(sample_rate, buffer_size)?;

    println!("Plugin initialized successfully!");
    println!();

    // Show parameter information
    let param_count = plugin.parameter_count();
    println!("Plugin has {} parameters", param_count);

    if param_count > 0 {
        println!("\nFirst 5 parameters:");
        for i in 0..param_count.min(5) {
            if let Ok(info) = plugin.parameter_info(i) {
                let value = plugin.get_parameter(i).unwrap_or(0.0);
                println!(
                    "  [{}] {} = {:.2} (range: {:.2} - {:.2}) {}",
                    i, info.name, value, info.min, info.max, info.unit
                );
            }
        }
    }

    println!("\nGetting SubCategories...");
    let categories = plugin.get_categories();
    println!("Plugin categories: {categories:?}");

    println!("\nExample completed successfully!");

    Ok(())
}

#[cfg(any(
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "visionos"
))]
fn main() {
    println!("VST3 is not available on mobile platforms.");
    println!("This example only works on desktop platforms (macOS, Windows, Linux).");
}
