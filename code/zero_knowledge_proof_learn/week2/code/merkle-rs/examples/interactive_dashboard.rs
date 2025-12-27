//! Interactive Security Testing Dashboard
//!
//! This example demonstrates the full interactive dashboard experience
//! with real-time security analysis, educational content, and comprehensive
//! visualization capabilities.

use merkle_rs::security::{SecurityDashboard, DashboardConfig};

fn main() {
    println!("🚀 Starting Merkle Tree Security Dashboard...");
    println!("==========================================");

    // Create dashboard with default configuration
    let config = DashboardConfig {
        real_time_updates: true,
        update_interval: 500,
        max_history: 10,
        educational_mode: true,
        color_output: true,
    };

    let mut dashboard = SecurityDashboard::new(config);

    // Start the interactive session
    match dashboard.start_interactive_session() {
        Ok(()) => {
            println!("\n✅ Dashboard session completed successfully!");
        }
        Err(e) => {
            eprintln!("\n❌ Dashboard error: {:?}", e);
        }
    }
}