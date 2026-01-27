use crate::artifacts::WitnessWrapper;
use crate::error::Result;
use crate::fileio::load_json_file;
use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;

/// Generate a proof from witness and keys
#[derive(Parser, Debug)]
pub struct ProveCommand {
    /// Input witness JSON file
    #[arg(long)]
    pub witness: PathBuf,

    /// Proving key JSON file
    #[arg(long)]
    pub pk: PathBuf,

    /// Output proof JSON file
    #[arg(long, default_value = "proof.json")]
    pub output: PathBuf,

    /// Show timing information
    #[arg(long)]
    pub benchmark: bool,
}

pub fn run(cmd: ProveCommand) -> Result<()> {
    println!("Generating proof...");
    println!("  Witness: {}", cmd.witness.display());
    println!("  Proving key: {}", cmd.pk.display());
    println!("  Output: {}", cmd.output.display());

    // Load witness
    let witness_wrapper: WitnessWrapper = load_json_file(cmd.witness.clone())?;
    println!(
        "  Circuit type: {:?}",
        witness_wrapper.metadata.circuit_type
    );

    // Start timing if benchmarking
    let start = if cmd.benchmark {
        Some(Instant::now())
    } else {
        None
    };

    // TODO: Load proving key
    let _pk_data = load_json_file::<serde_json::Value>(cmd.pk.clone())?;

    // TODO: Call Week 8's prove function based on circuit type
    match witness_wrapper.metadata.circuit_type {
        crate::error::CircuitType::Identity => {
            println!("  Generating proof for identity circuit...");
            // Call: zk_groth16_snark::identity::prove()
        }
        crate::error::CircuitType::Membership => {
            println!("  Generating proof for membership circuit...");
            // Call: zk_groth16_snark::membership::prove()
        }
        crate::error::CircuitType::Privacy => {
            println!("  Generating proof for privacy circuit...");
            // Call: zk_groth16_snark::privacy::prove()
        }
    }

    // Print timing if benchmarking
    if let Some(start) = start {
        let duration = start.elapsed();
        println!("  Proving time: {:?}", duration);
    }

    // TODO: Save proof to output file
    println!("  Proof saved to: {}", cmd.output.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prove_command_defaults() {
        let cmd = ProveCommand {
            witness: PathBuf::from("witness.json"),
            pk: PathBuf::from("pk.json"),
            output: PathBuf::from("proof.json"),
            benchmark: false,
        };

        assert_eq!(cmd.output, PathBuf::from("proof.json"));
        assert_eq!(cmd.benchmark, false);
    }
}
