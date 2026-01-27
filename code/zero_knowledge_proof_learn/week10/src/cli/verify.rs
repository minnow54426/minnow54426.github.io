use crate::error::Result;
use crate::fileio::load_json_file;
use clap::Parser;
use colored::*;
use std::path::PathBuf;

/// Verify a proof against public inputs
#[derive(Parser, Debug)]
pub struct VerifyCommand {
    /// Proof JSON file
    #[arg(long)]
    pub proof: PathBuf,

    /// Verifying key JSON file
    #[arg(long)]
    pub vk: PathBuf,

    /// Public inputs JSON file
    #[arg(long)]
    pub public: PathBuf,
}

pub fn run(cmd: VerifyCommand) -> Result<()> {
    println!("Verifying proof...");
    println!("  Proof: {}", cmd.proof.display());
    println!("  Verifying key: {}", cmd.vk.display());
    println!("  Public inputs: {}", cmd.public.display());

    // Load proof
    let proof_data = load_json_file::<serde_json::Value>(cmd.proof.clone())?;
    let circuit_type_str = proof_data["metadata"]["circuit_type"]
        .as_str()
        .ok_or_else(|| {
            crate::error::CliError::InvalidJson(
                cmd.proof.to_string_lossy().to_string(),
                "Missing circuit_type in metadata".to_string(),
            )
        })?;

    let circuit_type: crate::error::CircuitType = circuit_type_str.parse().map_err(|e| {
        crate::error::CliError::InvalidJson(cmd.proof.to_string_lossy().to_string(), e)
    })?;

    println!("  Circuit type: {:?}", circuit_type);

    // Load verifying key
    let _vk_data = load_json_file::<serde_json::Value>(cmd.vk.clone())?;

    // Load public inputs
    let _public_data = load_json_file::<serde_json::Value>(cmd.public.clone())?;

    // TODO: Call Week 8's verify function based on circuit type
    let verification_result = match circuit_type {
        crate::error::CircuitType::Identity => {
            println!("  Verifying identity proof...");
            // Call: zk_groth16_snark::identity::verify()
            true // TODO: Replace with actual verification
        }
        crate::error::CircuitType::Membership => {
            println!("  Verifying membership proof...");
            // Call: zk_groth16_snark::membership::verify()
            true // TODO: Replace with actual verification
        }
        crate::error::CircuitType::Privacy => {
            println!("  Verifying privacy proof...");
            // Call: zk_groth16_snark::privacy::verify()
            true // TODO: Replace with actual verification
        }
    };

    if verification_result {
        println!("{}", "✓ Proof verified".green());
    } else {
        println!("{}", "✗ Verification failed".red());
        return Err(crate::error::CliError::VerifyFailed(
            "Proof verification failed".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_command_parsing() {
        let cmd = VerifyCommand {
            proof: PathBuf::from("proof.json"),
            vk: PathBuf::from("vk.json"),
            public: PathBuf::from("public.json"),
        };

        assert_eq!(cmd.proof, PathBuf::from("proof.json"));
    }
}
