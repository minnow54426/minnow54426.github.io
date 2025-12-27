/// Hash CLI - A command-line tool for hashing data using the rust-protocol-basics library
///
/// This CLI demonstrates practical usage of the library's hashing capabilities
/// with a clean, user-friendly interface.
///
/// Examples:
///   # Hash a string
///   hash-cli "hello world"
///
///   # Hash from stdin
///   echo "test" | hash-cli
///
///   # Show hex encoding/decoding
///   hash-cli --encode "hello"
///   hash-cli --decode "68656c6c6f"
///
///   # Multiple inputs
///   hash-cli "item1" "item2" "item3" --merkle

use anyhow::{Context, Result};
use clap::{ArgGroup, Parser};
use rust_protocol_basics::*;
use std::io::{self, Read, Write};

/// Command-line arguments for the hash CLI
#[derive(Parser, Debug)]
#[command(
    name = "hash-cli",
    version = "0.1.0",
    author = "Rust Protocol Basics",
    about = "A CLI tool for hashing data using SHA-256",
    long_about = "A command-line interface to the rust-protocol-basics library, providing SHA-256 hashing, hex encoding/decoding, and Merkle tree generation."
)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["text", "stdin"])
))]
struct Args {
    /// Text to hash (can be specified multiple times)
    #[arg(
        short = 't',
        long = "text",
        value_name = "TEXT",
        help = "Text string(s) to hash"
    )]
    text: Vec<String>,

    /// Read from stdin instead of command line arguments
    #[arg(
        short = 's',
        long = "stdin",
        help = "Read input from stdin"
    )]
    stdin: bool,

    /// Encode input as hex instead of hashing
    #[arg(
        short = 'e',
        long = "encode",
        help = "Encode input as hex (no hashing)"
    )]
    encode: bool,

    /// Decode hex input to bytes
    #[arg(
        short = 'd',
        long = "decode",
        help = "Decode hex input to bytes"
    )]
    decode: bool,

    /// Generate Merkle tree for multiple inputs
    #[arg(
        short = 'm',
        long = "merkle",
        help = "Generate Merkle tree for multiple inputs"
    )]
    merkle: bool,

    /// Show output as uppercase hex
    #[arg(
        short = 'u',
        long = "upper",
        help = "Display output in uppercase hex"
    )]
    upper: bool,

    /// Double hash (SHA-256 of SHA-256)
    #[arg(
        short = '2',
        long = "double",
        help = "Apply SHA-256 twice (like Bitcoin)"
    )]
    double: bool,

    /// Quiet mode - only output the hash
    #[arg(
        short = 'q',
        long = "quiet",
        help = "Only output the hash, no labels"
    )]
    quiet: bool,

    /// Verbose mode - show detailed information
    #[arg(
        short = 'v',
        long = "verbose",
        help = "Show detailed processing information"
    )]
    verbose: bool,
}

/// Main CLI function
fn main() -> Result<()> {
    let args = Args::parse();

    // Validate argument combinations
    if args.encode && args.decode {
        anyhow::bail!("Cannot use --encode and --decode simultaneously");
    }

    if args.merkle && args.text.len() < 2 {
        anyhow::bail!("Merkle tree requires at least 2 inputs");
    }

    // Collect input data
    let inputs = collect_inputs(&args)?;

    // Process based on operation mode
    if args.encode {
        handle_encoding(&inputs, &args)?;
    } else if args.decode {
        handle_decoding(&inputs, &args)?;
    } else if args.merkle {
        handle_merkle(&inputs, &args)?;
    } else {
        handle_hashing(&inputs, &args)?;
    }

    Ok(())
}

/// Collect input data from arguments or stdin
fn collect_inputs(args: &Args) -> Result<Vec<Vec<u8>>> {
    if args.stdin {
        if args.verbose {
            eprintln!("Reading input from stdin...");
        }

        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read from stdin")?;

        // Remove trailing newline without extra allocation
        let trimmed = buffer.trim_end();
        Ok(vec![trimmed.as_bytes().to_vec()])
    } else {
        // Pre-allocate with exact capacity to avoid reallocations
        let mut inputs = Vec::with_capacity(args.text.len());
        for text in &args.text {
            inputs.push(text.as_bytes().to_vec());
        }
        Ok(inputs)
    }
}

/// Handle hex encoding
fn handle_encoding(inputs: &[Vec<u8>], args: &Args) -> Result<()> {
    for (i, input) in inputs.iter().enumerate() {
        let encoded = hex_encode(input)?;

        if args.quiet {
            println!("{}", encoded);
        } else if inputs.len() > 1 {
            println!("Input {}: {}", i + 1, encoded);
        } else {
            println!("{}", encoded);
        }

        if args.verbose {
            eprintln!("Encoded {} bytes to hex", input.len());
        }
    }

    Ok(())
}

/// Handle hex decoding
fn handle_decoding(inputs: &[Vec<u8>], args: &Args) -> Result<()> {
    for (i, input) in inputs.iter().enumerate() {
        let hex_str = String::from_utf8_lossy(input);
        let decoded = hex_decode(&hex_str)?;

        if args.quiet {
            // Output as raw bytes (may not be printable)
            io::stdout().write_all(&decoded)?;
        } else {
            let output = String::from_utf8_lossy(&decoded);
            if inputs.len() > 1 {
                println!("Input {}: {}", i + 1, output);
            } else {
                println!("{}", output);
            }
        }

        if args.verbose {
            eprintln!("Decoded hex string to {} bytes", decoded.len());
        }
    }

    Ok(())
}

/// Handle regular hashing
fn handle_hashing(inputs: &[Vec<u8>], args: &Args) -> Result<()> {
    for (i, input) in inputs.iter().enumerate() {
        let hash = if args.double {
            sha256d(input)
        } else {
            sha256(input)
        };

        let hash32 = Hash32::new(hash);

        let output = if args.upper {
            format!("{:X}", hash32)
        } else {
            format!("{}", hash32)
        };

        if args.quiet {
            println!("{}", output);
        } else if inputs.len() > 1 {
            println!("Input {}: {}", i + 1, output);
        } else {
            println!("{}", output);
        }

        if args.verbose {
            eprintln!("Hashed {} bytes using SHA-256{}", input.len(), if args.double { "²" } else { "" });
        }
    }

    Ok(())
}

/// Handle Merkle tree generation
fn handle_merkle(inputs: &[Vec<u8>], args: &Args) -> Result<()> {
    if args.verbose {
        eprintln!("Building Merkle tree with {} leaves...", inputs.len());
    }

    // Convert inputs to byte slices for MerkleTree
    let input_refs: Vec<&[u8]> = inputs.iter().map(|v| v.as_slice()).collect();
    let tree = MerkleTree::new(&input_refs);

    let root_hash = Hash32::new(tree.root());
    let root_output = if args.upper {
        format!("{:X}", root_hash)
    } else {
        format!("{}", root_hash)
    };

    if args.quiet {
        println!("{}", root_output);
    } else {
        println!("Merkle Root: {}", root_output);
        println!("Leaf Count: {}", tree.leaf_count());

        if args.verbose {
            eprintln!("Merkle tree constructed successfully");
            eprintln!("Each leaf hash: SHA-256(input)");
            eprintln!("Tree construction: Pairwise hashing up to root");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_command() {
        // Test that the CLI command can be created
        Args::command().debug_assert();
    }

    #[test]
    fn test_collect_inputs_from_args() {
        let args = Args {
            text: vec!["hello".to_string(), "world".to_string()],
            stdin: false,
            encode: false,
            decode: false,
            merkle: false,
            upper: false,
            double: false,
            quiet: false,
            verbose: false,
        };

        let inputs = collect_inputs(&args).unwrap();
        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[0], b"hello");
        assert_eq!(inputs[1], b"world");
    }
}