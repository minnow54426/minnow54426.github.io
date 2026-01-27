use anyhow::Result;
use clap::Parser;
use zk_proof_artifacts::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        zk_proof_artifacts::cli::Commands::Setup(cmd) => {
            zk_proof_artifacts::cli::setup::run(cmd)?;
        }
        zk_proof_artifacts::cli::Commands::Prove(cmd) => {
            zk_proof_artifacts::cli::prove::run(cmd)?;
        }
        zk_proof_artifacts::cli::Commands::Verify(cmd) => {
            zk_proof_artifacts::cli::verify::run(cmd)?;
        }
    }

    Ok(())
}
