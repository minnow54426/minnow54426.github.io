pub mod setup;
pub mod prove;
pub mod verify;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zk-artifacts")]
#[command(about = "CLI tool for ZK-SNARK proof artifacts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate proving/verifying keys for a circuit
    Setup(setup::SetupCommand),
    /// Generate a proof from witness and keys
    Prove(prove::ProveCommand),
    /// Verify a proof against public inputs
    Verify(verify::VerifyCommand),
}
