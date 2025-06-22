use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use crate::partitioned_gc::{Circuit, GarbledGate, WireKeys};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct PublicCircuitData {
    circuit: Circuit,
    wire_keys: HashMap<usize, WireKeys>,
    garbled_gates: Vec<GarbledGate>,
}

pub fn publish_to_public_repo(
    garbled_gates: &Vec<GarbledGate>,
    wire_keys: &HashMap<usize, WireKeys>,
    circuit: &Circuit,
) -> std::io::Result<()> {
    // Create output directory if it doesn't exist
    let output_dir = Path::new("public_repo/");
    create_dir_all(&output_dir)?;

    // Clone data so we can serialize it
    let data = PublicCircuitData {
        circuit: circuit.clone(),
        wire_keys: wire_keys.clone(),
        garbled_gates: garbled_gates.clone(),
    };

    // Serialize data to pretty JSON format
    let serialized = serde_json::to_string_pretty(&data)
        .expect("Failed to serialize public circuit data");

    // Write JSON string to file
    let mut file = File::create(output_dir.join("published_circuit.json"))?;
    file.write_all(serialized.as_bytes())?;

    println!("\n[Public Repo] Published garbled circuit to public_repo/published_circuit.json");
    Ok(())
}
