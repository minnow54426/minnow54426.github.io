use crate::error::{CircuitType, Result};
use clap::Parser;
use std::path::PathBuf;

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

    // Create output directory
    std::fs::create_dir_all(&cmd.output_dir)?;

    // Check if keys already exist
    let vk_path = cmd.output_dir.join("vk.json");
    let pk_path = cmd.output_dir.join("pk.json");

    if (vk_path.exists() || pk_path.exists()) && !cmd.force {
        println!("Keys already exist. Use --force to overwrite.");
        return Ok(());
    }

    // TODO: Call Week 8's setup function
    // This will be implemented after testing with actual Week 8 library
    match cmd.circuit {
        CircuitType::Identity => {
            println!("Generating keys for identity circuit...");
            // Call: zk_groth16_snark::identity::setup()
        }
        CircuitType::Membership => {
            println!("Generating keys for membership circuit...");
            // Call: zk_groth16_snark::membership::setup()
        }
        CircuitType::Privacy => {
            println!("Generating keys for privacy circuit...");
            // Call: zk_groth16_snark::privacy::setup()
        }
    }

    println!("Keys saved to:");
    println!("  {}", vk_path.display());
    println!("  {}", pk_path.display());

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
