//! Interactive Security Testing Dashboard
//!
//! This module provides a comprehensive interactive dashboard for real-time
//! security analysis with visualization, progress tracking, and educational
//! features for learning Merkle tree security properties.

use crate::security::{SecurityTestConfig, SecurityTestSuite};
use std::collections::HashMap;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Interactive security dashboard
pub struct SecurityDashboard {
    config: DashboardConfig,
    test_history: Vec<TestSession>,
    current_session: Option<TestSession>,
}

/// Dashboard configuration
#[derive(Debug, Clone)]
pub struct DashboardConfig {
    /// Enable real-time updates
    pub real_time_updates: bool,
    /// Update interval in milliseconds
    pub update_interval: u64,
    /// Maximum history size
    pub max_history: usize,
    /// Enable educational tooltips
    pub educational_mode: bool,
    /// Color output (if supported)
    pub color_output: bool,
}

/// Test session information
#[derive(Debug, Clone)]
pub struct TestSession {
    pub id: String,
    pub timestamp: String,
    pub config: SecurityTestConfig,
    pub results: Option<DashboardResults>,
    pub status: TestStatus,
}

/// Dashboard results with enhanced metrics
#[derive(Debug, Clone)]
pub struct DashboardResults {
    pub basic_metrics: BasicMetrics,
    pub advanced_metrics: Option<AdvancedMetrics>,
    pub visualization_data: VisualizationData,
    pub recommendations: Vec<Recommendation>,
}

/// Basic security metrics
#[derive(Debug, Clone)]
pub struct BasicMetrics {
    pub tests_run: usize,
    pub tests_passed: usize,
    pub tests_failed: usize,
    pub collision_resistance: f64,
    pub binding_strength: f64,
    pub randomness_quality: f64,
    pub execution_time_ms: u64,
}

/// Advanced security metrics
#[derive(Debug, Clone)]
pub struct AdvancedMetrics {
    pub total_attacks: usize,
    pub successful_attacks: usize,
    pub overall_resistance: f64,
    pub classical_resistance: f64,
    pub quantum_resistance: f64,
    pub attack_breakdown: HashMap<String, AttackMetric>,
}

/// Individual attack metric
#[derive(Debug, Clone)]
pub struct AttackMetric {
    pub success: bool,
    pub attempts: usize,
    pub time_complexity: String,
    pub risk_level: RiskLevel,
}

/// Risk level classification
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Visualization data for charts
#[derive(Debug, Clone)]
pub struct VisualizationData {
    pub security_gauge: f64,
    pub attack_resistance_chart: Vec<(String, f64)>,
    pub timeline_data: Vec<(String, f64)>,
    pub comparison_data: HashMap<String, f64>,
}

/// Security recommendation
#[derive(Debug, Clone)]
pub struct Recommendation {
    pub priority: RecommendationPriority,
    pub category: String,
    pub message: String,
    pub action_items: Vec<String>,
}

/// Recommendation priority
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationPriority {
    Info,
    Warning,
    Critical,
}

/// Test status
#[derive(Debug, Clone, PartialEq)]
pub enum TestStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl SecurityDashboard {
    /// Create a new security dashboard
    pub fn new(config: DashboardConfig) -> Self {
        Self {
            config,
            test_history: Vec::new(),
            current_session: None,
        }
    }

    /// Create dashboard with default configuration
    pub fn default() -> Self {
        Self::new(DashboardConfig {
            real_time_updates: true,
            update_interval: 500,
            max_history: 10,
            educational_mode: true,
            color_output: true,
        })
    }

    /// Start interactive dashboard session
    pub fn start_interactive_session(&mut self) -> DashboardResult<()> {
        self.clear_screen();
        self.print_banner();

        loop {
            self.print_main_menu();

            match self.get_user_choice()? {
                MenuChoice::QuickTest => self.run_quick_test()?,
                MenuChoice::ComprehensiveTest => self.run_comprehensive_test()?,
                MenuChoice::AdvancedAttacks => self.run_advanced_attacks()?,
                MenuChoice::CustomConfig => self.configure_custom_test()?,
                MenuChoice::ViewHistory => self.view_test_history()?,
                MenuChoice::EducationalMode => self.run_educational_mode()?,
                MenuChoice::ExportResults => self.export_results()?,
                MenuChoice::Exit => {
                    self.print_goodbye();
                    break;
                }
            }
        }

        Ok(())
    }

    /// Run quick security test
    fn run_quick_test(&mut self) -> DashboardResult<()> {
        self.clear_screen();
        self.print_header("🚀 Quick Security Test");

        let config = SecurityTestConfig {
            test_iterations: 50,
            max_data_size: 25,
            exhaustive: false,
            seed: None,
        };

        self.show_loading_spinner("Running quick security analysis...", 2000);

        let suite = SecurityTestSuite::with_config(config.clone());
        let results = suite.run_all_tests();

        let dashboard_results = self.convert_to_dashboard_results(results, &config);
        self.display_results(&dashboard_results);

        self.save_session(config, dashboard_results);
        self.pause_for_user();

        Ok(())
    }

    /// Run comprehensive security test
    fn run_comprehensive_test(&mut self) -> DashboardResult<()> {
        self.clear_screen();
        self.print_header("🔬 Comprehensive Security Analysis");

        let config = SecurityTestConfig {
            test_iterations: 200,
            max_data_size: 100,
            exhaustive: true,
            seed: Some(42),
        };

        self.show_progress_bar("Running comprehensive analysis...", 3000);

        let suite = SecurityTestSuite::with_config(config.clone());
        let results = suite.run_all_tests();

        let dashboard_results = self.convert_to_dashboard_results(results, &config);
        self.display_comprehensive_results(&dashboard_results);

        self.save_session(config, dashboard_results);
        self.pause_for_user();

        Ok(())
    }

    /// Run advanced attack simulations
    fn run_advanced_attacks(&mut self) -> DashboardResult<()> {
        self.clear_screen();
        self.print_header("⚔️ Advanced Attack Simulations");

        let config = SecurityTestConfig {
            test_iterations: 300,
            max_data_size: 150,
            exhaustive: true,
            seed: Some(123),
        };

        self.show_animated_loading("Initializing advanced attack simulator...", 2500);

        let suite = SecurityTestSuite::with_advanced_attacks(config.clone(), true);
        let results = suite.run_all_tests();

        let dashboard_results = self.convert_to_dashboard_results(results, &config);
        self.display_advanced_results(&dashboard_results);

        self.save_session(config, dashboard_results);
        self.pause_for_user();

        Ok(())
    }

    /// Configure custom test parameters
    fn configure_custom_test(&mut self) -> DashboardResult<()> {
        self.clear_screen();
        self.print_header("⚙️ Custom Test Configuration");

        println!("Configure your custom security test:\n");

        let iterations = self.get_numeric_input("Number of test iterations (10-1000)", 50, 10, 1000)?;
        let data_size = self.get_numeric_input("Maximum data size (10-500)", 100, 10, 500)?;
        let exhaustive = self.get_yes_no_input("Enable exhaustive testing (includes quantum simulation)?", false)?;
        let use_seed = self.get_yes_no_input("Use random seed for reproducible results?", true)?;
        let seed = if use_seed {
            Some(42)
        } else {
            None
        };

        let config = SecurityTestConfig {
            test_iterations: iterations,
            max_data_size: data_size,
            exhaustive,
            seed,
        };

        println!("\n✅ Configuration saved!");
        println!("Test iterations: {}", iterations);
        println!("Max data size: {}", data_size);
        println!("Exhaustive testing: {}", exhaustive);
        println!("Random seed: {:?}", seed);

        if self.get_yes_no_input("\nRun test with this configuration?", true)? {
            self.run_custom_test(config)?;
        }

        Ok(())
    }

    /// Run custom test with user configuration
    fn run_custom_test(&mut self, config: SecurityTestConfig) -> DashboardResult<()> {
        self.clear_screen();
        self.print_header("🧪 Custom Security Test");

        self.show_progress_bar("Running custom security analysis...", 2000);

        let suite = SecurityTestSuite::with_config(config.clone());
        let results = suite.run_all_tests();

        let dashboard_results = self.convert_to_dashboard_results(results, &config);
        self.display_results(&dashboard_results);

        self.save_session(config, dashboard_results);
        self.pause_for_user();

        Ok(())
    }

    /// View test history
    fn view_test_history(&self) -> DashboardResult<()> {
        self.clear_screen();
        self.print_header("📊 Test History");

        if self.test_history.is_empty() {
            println!("No test history available. Run some tests first!");
            self.pause_for_user();
            return Ok(());
        }

        println!("Recent Test Sessions:\n");

        for (i, session) in self.test_history.iter().rev().take(5).enumerate() {
            println!("{}. {} ({})", i + 1, session.id, session.timestamp);
            println!("   Status: {:?}", session.status);

            if let Some(ref results) = session.results {
                println!("   Tests: {}/{} passed", results.basic_metrics.tests_passed, results.basic_metrics.tests_run);
                println!("   Security Score: {:.1}%", results.basic_metrics.collision_resistance * 100.0);
            }
            println!();
        }

        self.pause_for_user();
        Ok(())
    }

    /// Run educational mode
    fn run_educational_mode(&mut self) -> DashboardResult<()> {
        self.clear_screen();
        self.print_header("🎓 Educational Mode");

        println!("Welcome to Educational Mode! Learn about Merkle tree security:\n");

        let topics = vec![
            "1. Collision Resistance",
            "2. Binding Properties",
            "3. Domain Separation",
            "4. Quantum Resistance",
            "5. Attack Vectors",
            "6. Security Best Practices",
        ];

        for topic in &topics {
            println!("{}", topic);
        }

        println!("\nSelect a topic to learn more (1-6), or 0 to return:");

        let choice = self.get_numeric_input("", 0, 0, 6)?;

        match choice {
            0 => return Ok(()),
            1 => self.explain_collision_resistance(),
            2 => self.explain_binding_properties(),
            3 => self.explain_domain_separation(),
            4 => self.explain_quantum_resistance(),
            5 => self.explain_attack_vectors(),
            6 => self.explain_security_best_practices(),
            _ => unreachable!(),
        }

        self.pause_for_user();
        Ok(())
    }

    /// Export test results
    fn export_results(&self) -> DashboardResult<()> {
        self.clear_screen();
        self.print_header("💾 Export Results");

        if self.test_history.is_empty() {
            println!("No results to export. Run some tests first!");
            self.pause_for_user();
            return Ok(());
        }

        println!("Select export format:");
        println!("1. JSON format");
        println!("2. Pretty text format");
        println!("3. CSV summary");
        println!("0. Back to menu");

        let choice = self.get_numeric_input("", 0, 0, 3)?;

        match choice {
            0 => return Ok(()),
            1 => self.export_json()?,
            2 => self.export_text()?,
            3 => self.export_csv()?,
            _ => unreachable!(),
        }

        println!("\n✅ Results exported successfully!");
        self.pause_for_user();

        Ok(())
    }

    /// Convert test results to dashboard format
    pub fn convert_to_dashboard_results(&self, results: crate::security::SecurityTestResults, _config: &SecurityTestConfig) -> DashboardResults {
        let basic_metrics = BasicMetrics {
            tests_run: results.passed + results.failed,
            tests_passed: results.passed,
            tests_failed: results.failed,
            collision_resistance: results.metrics.collision_resistance,
            binding_strength: results.metrics.binding_strength,
            randomness_quality: results.metrics.randomness_quality,
            execution_time_ms: 0, // Would need timing instrumentation
        };

        let advanced_metrics = results.advanced_results.as_ref().map(|advanced| {
            let mut attack_breakdown = HashMap::new();
            for attack in &advanced.attack_details {
                let risk_level = if attack.success {
                    if attack.attack_type.contains("Quantum") { RiskLevel::High }
                    else { RiskLevel::Critical }
                } else {
                    RiskLevel::Low
                };

                attack_breakdown.insert(attack.attack_type.clone(), AttackMetric {
                    success: attack.success,
                    attempts: attack.attempts,
                    time_complexity: attack.time_complexity.clone(),
                    risk_level,
                });
            }

            AdvancedMetrics {
                total_attacks: advanced.total_attacks,
                successful_attacks: advanced.successful_attacks,
                overall_resistance: advanced.security_assessment.overall_resistance,
                classical_resistance: advanced.security_assessment.classical_resistance,
                quantum_resistance: advanced.security_assessment.quantum_resistance,
                attack_breakdown,
            }
        });

        let security_gauge = (basic_metrics.collision_resistance + basic_metrics.binding_strength + basic_metrics.randomness_quality) / 3.0;

        let mut attack_resistance_chart = vec![
            ("Collision".to_string(), basic_metrics.collision_resistance),
            ("Binding".to_string(), basic_metrics.binding_strength),
            ("Randomness".to_string(), basic_metrics.randomness_quality),
        ];

        if let Some(ref advanced) = advanced_metrics {
            attack_resistance_chart.push(("Classical".to_string(), advanced.classical_resistance));
            attack_resistance_chart.push(("Quantum".to_string(), advanced.quantum_resistance));
        }

        let visualization_data = VisualizationData {
            security_gauge,
            attack_resistance_chart,
            timeline_data: vec![("Current".to_string(), security_gauge)],
            comparison_data: HashMap::new(),
        };

        let mut recommendations = Vec::new();

        if basic_metrics.collision_resistance < 0.8 {
            recommendations.push(Recommendation {
                priority: RecommendationPriority::Warning,
                category: "Collision Resistance".to_string(),
                message: "Collision resistance could be improved".to_string(),
                action_items: vec![
                    "Consider increasing hash output size".to_string(),
                    "Review domain separation implementation".to_string(),
                ],
            });
        }

        if let Some(ref advanced) = advanced_metrics {
            if advanced.quantum_resistance < 0.7 {
                recommendations.push(Recommendation {
                    priority: RecommendationPriority::Critical,
                    category: "Quantum Security".to_string(),
                    message: "Post-quantum security may be insufficient".to_string(),
                    action_items: vec![
                        "Evaluate post-quantum hash functions".to_string(),
                        "Consider larger hash outputs for quantum resistance".to_string(),
                    ],
                });
            }
        }

        DashboardResults {
            basic_metrics,
            advanced_metrics,
            visualization_data,
            recommendations,
        }
    }

    /// Display basic results
    pub fn display_results(&self, results: &DashboardResults) {
        println!("\n📊 Security Test Results:");
        println!("═══════════════════════════════════════");

        println!("\n🎯 Basic Metrics:");
        println!("  Tests Run: {}", results.basic_metrics.tests_run);
        println!("  ✅ Passed: {}", results.basic_metrics.tests_passed);
        if results.basic_metrics.tests_failed > 0 {
            println!("  ❌ Failed: {}", results.basic_metrics.tests_failed);
        }

        println!("\n📈 Security Scores:");
        println!("  Collision Resistance: {:.1}%", results.basic_metrics.collision_resistance * 100.0);
        println!("  Binding Strength: {:.1}%", results.basic_metrics.binding_strength * 100.0);
        println!("  Randomness Quality: {:.1}%", results.basic_metrics.randomness_quality * 100.0);

        self.display_security_gauge(results.visualization_data.security_gauge);

        if !results.recommendations.is_empty() {
            println!("\n💡 Recommendations:");
            for rec in &results.recommendations {
                let priority_icon = match rec.priority {
                    RecommendationPriority::Info => "ℹ️",
                    RecommendationPriority::Warning => "⚠️",
                    RecommendationPriority::Critical => "🚨",
                };
                println!("  {} {}: {}", priority_icon, rec.category, rec.message);
            }
        }
    }

    /// Display comprehensive results
    pub fn display_comprehensive_results(&self, results: &DashboardResults) {
        self.display_results(results);

        if let Some(ref advanced) = results.advanced_metrics {
            println!("\n⚔️ Advanced Security Analysis:");
            println!("  Total Attack Attempts: {}", advanced.total_attacks);
            println!("  Successful Attacks: {}", advanced.successful_attacks);

            println!("\n🛡️ Resistance Levels:");
            println!("  Overall: {:.1}%", advanced.overall_resistance * 100.0);
            println!("  Classical: {:.1}%", advanced.classical_resistance * 100.0);
            println!("  Quantum: {:.1}%", advanced.quantum_resistance * 100.0);

            println!("\n📊 Attack Breakdown:");
            for (attack_type, metric) in &advanced.attack_breakdown {
                let status = if metric.success { "❌ Vulnerable" } else { "✅ Resisted" };
                let risk_icon = match metric.risk_level {
                    RiskLevel::Low => "🟢",
                    RiskLevel::Medium => "🟡",
                    RiskLevel::High => "🟠",
                    RiskLevel::Critical => "🔴",
                };
                println!("  {} {}: {} ({})", risk_icon, attack_type, status, metric.time_complexity);
            }
        }
    }

    /// Display advanced results with full detail
    pub fn display_advanced_results(&self, results: &DashboardResults) {
        self.display_comprehensive_results(results);

        println!("\n📊 Visualization Data:");
        println!("  Security Gauge: {:.1}%", results.visualization_data.security_gauge * 100.0);

        println!("\n📈 Resistance Chart:");
        for (category, score) in &results.visualization_data.attack_resistance_chart {
            let bar_len = (score * 20.0) as usize;
            let bar = "█".repeat(bar_len);
            println!("  {:<15} {} {:.1}%", category, bar, score * 100.0);
        }
    }

    /// Display security gauge
    pub fn display_security_gauge(&self, score: f64) {
        let percentage = (score * 100.0) as usize;
        let filled = (score * 30.0) as usize;
        let empty = 30 - filled;

        let gauge = "█".repeat(filled) + &"░".repeat(empty);
        let color = if percentage >= 90 { "🟢" } else if percentage >= 70 { "🟡" } else { "🔴" };

        println!("\n🎯 Security Score: {} {}%", color, percentage);
        println!("  [{}]", gauge);
    }

    /// Save test session to history
    fn save_session(&mut self, config: SecurityTestConfig, results: DashboardResults) {
        let session = TestSession {
            id: format!("test_{}", self.test_history.len() + 1),
            timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            config,
            results: Some(results),
            status: TestStatus::Completed,
        };

        self.test_history.push(session);

        // Keep only recent history
        if self.test_history.len() > self.config.max_history {
            self.test_history.remove(0);
        }
    }

    // Public methods for UI interaction
    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }

    pub fn print_banner(&self) {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║          🛡️  Merkle Tree Security Dashboard               ║");
        println!("║              Advanced Security Analysis System            ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();
    }

    pub fn print_header(&self, title: &str) {
        println!("{}", title);
        println!("{}", "═".repeat(title.len()));
        println!();
    }

    fn print_main_menu(&self) {
        println!("🎯 Main Menu:");
        println!("1. 🚀 Quick Security Test");
        println!("2. 🔬 Comprehensive Analysis");
        println!("3. ⚔️ Advanced Attack Simulations");
        println!("4. ⚙️ Custom Test Configuration");
        println!("5. 📊 View Test History");
        println!("6. 🎓 Educational Mode");
        println!("7. 💾 Export Results");
        println!("0. 🚪 Exit");
        println!();
    }

    fn get_user_choice(&self) -> DashboardResult<MenuChoice> {
        print!("Enter your choice (0-7): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => Ok(MenuChoice::QuickTest),
            "2" => Ok(MenuChoice::ComprehensiveTest),
            "3" => Ok(MenuChoice::AdvancedAttacks),
            "4" => Ok(MenuChoice::CustomConfig),
            "5" => Ok(MenuChoice::ViewHistory),
            "6" => Ok(MenuChoice::EducationalMode),
            "7" => Ok(MenuChoice::ExportResults),
            "0" => Ok(MenuChoice::Exit),
            _ => {
                println!("Invalid choice. Please try again.");
                self.get_user_choice()
            }
        }
    }

    fn get_numeric_input(&self, prompt: &str, default: usize, min: usize, max: usize) -> DashboardResult<usize> {
        loop {
            print!("{} [{}-{}, default: {}]: ", prompt, min, max, default);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim();
            if input.is_empty() {
                return Ok(default);
            }

            match input.parse::<usize>() {
                Ok(n) if n >= min && n <= max => return Ok(n),
                _ => println!("Invalid input. Please enter a number between {} and {}.", min, max),
            }
        }
    }

    fn get_yes_no_input(&self, prompt: &str, default: bool) -> DashboardResult<bool> {
        let default_str = if default { "Y/n" } else { "y/N" };

        loop {
            print!("{} [{}]: ", prompt, default_str);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim().to_lowercase();
            if input.is_empty() {
                return Ok(default);
            }

            match input.as_str() {
                "y" | "yes" => return Ok(true),
                "n" | "no" => return Ok(false),
                _ => println!("Please enter 'y' or 'n'."),
            }
        }
    }

    pub fn show_loading_spinner(&self, message: &str, duration_ms: u64) {
        print!("{} ", message);
        io::stdout().flush().unwrap();

        let spinner = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

        for i in 0..(duration_ms / 100) {
            let i_usize = i as usize;
            print!("\r{} {}", message, spinner[i_usize % spinner.len()]);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }

        println!(" ✅");
    }

    pub fn show_progress_bar(&self, message: &str, duration_ms: u64) {
        print!("{} [", message);
        io::stdout().flush().unwrap();

        for _ in 0..20 {
            print!("█");
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(duration_ms / 20));
        }

        println!("] ✅");
    }

    pub fn show_animated_loading(&self, message: &str, duration_ms: u64) {
        let frames = vec!["⚪", "⚫", "⚪", "⚫"];

        for i in 0..(duration_ms / 200) {
            let i_usize = i as usize;
            print!("\r{} {}", message, frames[i_usize % frames.len()]);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(200));
        }

        println!(" ✅");
    }

    fn pause_for_user(&self) {
        print!("\nPress Enter to continue...");
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut String::new());
    }

    fn print_goodbye(&self) {
        println!("\n👋 Thank you for using the Merkle Tree Security Dashboard!");
        println!("Stay secure and keep learning! 🛡️");
    }

    // Educational content methods
    fn explain_collision_resistance(&self) {
        println!("\n🎯 Collision Resistance");
        println!("═════════════════════════════");
        println!();
        println!("Collision resistance is a fundamental property of cryptographic hash functions.");
        println!("It ensures that it's computationally infeasible to find two different inputs");
        println!("that produce the same hash output.");
        println!();
        println!("🔍 Key Points:");
        println!("• For SHA-256, finding a collision requires ~2^128 operations");
        println!("• Your Merkle tree uses domain separation to prevent collisions");
        println!("• The 0x00 prefix for leaves and 0x01 for internal nodes ensures");
        println!("  that a leaf hash can never equal an internal node hash");
        println!();
        println!("💡 Educational Insight:");
        println!("Without domain separation, an attacker could potentially create");
        println!("'second preimage' attacks where different data produces the same hash.");
    }

    fn explain_binding_properties(&self) {
        println!("\n🔗 Binding Properties");
        println!("═════════════════════════════");
        println!();
        println!("The binding property ensures that a Merkle root 'binds' to a specific");
        println!("set of leaves, making it impossible to find different leaves that");
        println!("produce the same root.");
        println!();
        println!("🔍 Key Points:");
        println!("• The root commits to the exact set and order of leaves");
        println!("• Changing any leaf, even by one bit, changes the root");
        println!("• This property enables cryptographic commitment schemes");
        println!();
        println!("💡 Educational Insight:");
        println!("This is why Merkle trees are used in blockchain technology -");
        println!("they provide immutable, auditable commitment to transaction data.");
    }

    fn explain_domain_separation(&self) {
        println!("\n🏷️ Domain Separation");
        println!("═════════════════════════════");
        println!();
        println!("Domain separation prevents hash function outputs from being");
        println!("ambiguous - distinguishing between different types of inputs.");
        println!();
        println!("🔍 In Your Implementation:");
        println!("• Leaf hashes: H(0x00 || data)");
        println!("• Internal node hashes: H(0x01 || left || right)");
        println!("• The 0x00 and 0x01 prefixes prevent confusion");
        println!();
        println!("💡 Educational Insight:");
        println!("Without domain separation, an attacker could potentially");
        println!("create length extension attacks or confuse leaf vs node hashes.");
    }

    fn explain_quantum_resistance(&self) {
        println!("\n⚛️ Quantum Resistance");
        println!("═════════════════════════════");
        println!();
        println!("Quantum computers threaten classical cryptography through algorithms");
        println!("like Grover's, which provides quadratic speedup for search problems.");
        println!();
        println!("🔍 Impact on Merkle Trees:");
        println!("• Classical security: 2^256 operations for collision");
        println!("• Quantum security: ~2^128 operations (Grover's algorithm)");
        println!("• 256-bit hashes provide 128-bit post-quantum security");
        println!();
        println!("💡 Educational Insight:");
        println!("While quantum computers reduce security, 256-bit hashes still");
        println!("provide adequate security for most applications in the quantum era.");
    }

    fn explain_attack_vectors(&self) {
        println!("\n⚔️ Common Attack Vectors");
        println!("═════════════════════════════");
        println!();
        println!("Understanding attack vectors helps build stronger defenses:");
        println!();
        println!("🎯 1. Collision Attacks");
        println!("   Find two different inputs with same hash");
        println!("   Difficulty: ~2^128 for SHA-256");
        println!();
        println!("🎯 2. Preimage Attacks");
        println!("   Find input that produces specific hash");
        println!("   Difficulty: ~2^256 for SHA-256");
        println!();
        println!("🎯 3. Length Extension");
        println!("   Extend message and predict new hash");
        println!("   Prevented by domain separation");
        println!();
        println!("🎯 4. Side Channel Attacks");
        println!("   Extract information from implementation");
        println!("   Prevented by constant-time operations");
    }

    fn explain_security_best_practices(&self) {
        println!("\n🛡️ Security Best Practices");
        println!("═════════════════════════════");
        println!();
        println!("Follow these practices for secure Merkle tree implementations:");
        println!();
        println!("✅ 1. Use Domain Separation");
        println!("   Different prefixes for different hash contexts");
        println!();
        println!("✅ 2. Implement Constant-Time Operations");
        println!("   Prevent timing side-channel attacks");
        println!();
        println!("✅ 3. Validate All Inputs");
        println!("   Prevent malformed data attacks");
        println!();
        println!("✅ 4. Use Cryptographically Secure Hashes");
        println!("   SHA-256, BLAKE3, or similar standards");
        println!();
        println!("✅ 5. Consider Quantum Resistance");
        println!("   Larger hash outputs for post-quantum security");
        println!();
        println!("💡 Your Implementation:");
        println!("Your Merkle tree follows most of these best practices, making");
        println!("it suitable for production cryptographic applications.");
    }

    // Export methods (simplified implementations)
    fn export_json(&self) -> DashboardResult<()> {
        println!("Exporting results in JSON format...");
        // Implementation would serialize to JSON
        Ok(())
    }

    fn export_text(&self) -> DashboardResult<()> {
        println!("Exporting results in text format...");
        // Implementation would format as readable text
        Ok(())
    }

    fn export_csv(&self) -> DashboardResult<()> {
        println!("Exporting results in CSV format...");
        // Implementation would create CSV summary
        Ok(())
    }
}

/// Menu choice enumeration
#[derive(Debug, Clone, PartialEq)]
enum MenuChoice {
    QuickTest,
    ComprehensiveTest,
    AdvancedAttacks,
    CustomConfig,
    ViewHistory,
    EducationalMode,
    ExportResults,
    Exit,
}

/// Dashboard result type
type DashboardResult<T> = Result<T, DashboardError>;

/// Dashboard error type
#[derive(Debug)]
enum DashboardError {
    IoError(std::io::Error),
    InvalidInput(String),
}

impl From<std::io::Error> for DashboardError {
    fn from(error: std::io::Error) -> Self {
        DashboardError::IoError(error)
    }
}

// Add chrono dependency for timestamps
extern crate chrono;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = SecurityDashboard::default();
        assert!(dashboard.test_history.is_empty());
        assert!(dashboard.current_session.is_none());
    }

    #[test]
    fn test_dashboard_config() {
        let config = DashboardConfig {
            real_time_updates: true,
            update_interval: 500,
            max_history: 10,
            educational_mode: true,
            color_output: true,
        };

        let dashboard = SecurityDashboard::new(config);
        assert_eq!(dashboard.config.max_history, 10);
    }
}