use clap::Parser;
use std::path::PathBuf;
use crate::error::{CircuitType, Result};

/// Generate proving/verifying keys for a circuit
#[derive(Parser, Debug)]
pub struct SetupCommand {
    /// Circuit type: identity | membership | privacy
    #[arg(long)]
    pub circuit: CircuitType,

    /// Output directory for keys
    #[arg(long, default_value = "./keys")]
    pub output_dir: PathBuf,

    /// Overwrite existing keys
    #[arg(long)]
    pub force: bool,
}

pub fn run(cmd: SetupCommand) -> Result<()> {
    println!("Setting up circuit: {:?}", cmd.circuit);
    println!("Output directory: {}", cmd.output_dir.display());

    // TODO: Implement actual key generation in Task 8
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_command_parsing() {
        let cmd = SetupCommand {
            circuit: CircuitType::Identity,
            output_dir: PathBuf::from("./test_keys"),
            force: false,
        };

        assert_eq!(cmd.circuit, CircuitType::Identity);
        assert_eq!(cmd.output_dir, PathBuf::from("./test_keys"));
    }
}
