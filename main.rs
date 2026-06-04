use sha2::{Sha256, Digest};
use std::fs;
use std::env;

fn main() {
    // Accept optional filename argument, default to "mysequence.txt"
    let filename = env::args().nth(1).unwrap_or_else(|| "mysequence.txt".to_string());

    let data = fs::read(&filename).unwrap_or_else(|e| {
        eprintln!("Error reading '{}': {}", filename, e);
        std::process::exit(1);
    });

    let hash = hex::encode(Sha256::digest(&data));
    println!("SHA-256 hash of '{}':\n{}", filename, hash);
}
