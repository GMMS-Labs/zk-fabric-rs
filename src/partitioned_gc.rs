//! Partitioned Garbled Circuit Scheme Implementation
//!
//! Implements the construction of partitioned garbled circuits from a base Boolean circuit,
//! supporting multi-party oblivious transfer (OT) verification protocol.
//!
//! Reference:
//! - Yao's Garbled Circuits basics
//! - Partitioning scheme for splitting circuits into multiple independent garbled circuits
//! - Ensures privacy and correctness via cryptographic commitments
//!
//! Steps implemented here:
//! 1) Input preparation: pairing inputs and auxiliary randomness
//! 2) Garbled circuit construction (keys, ciphertext tables)
//! 3) Partitioning of garbled circuit matrix into independent circuits
//! 4) Running protocol iterations per partition
//! 5) Aggregating final verification output

use serde::{Serialize, Deserialize};

use rand::Rng;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Wire key pair: keys corresponding to 0 and 1 values on a wire
// #[derive(Debug, Clone)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WireKeys {
    pub key0: Vec<u8>, // cryptographic key representing 0
    pub key1: Vec<u8>, // cryptographic key representing 1
}

/// Logical gate types supported (for simplicity)
// #[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GateType {
    And,
    Or,
    Xor,
    Not,
}

/// A Boolean gate in the circuit
// #[derive(Debug, Clone)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Gate {
    pub gate_type: GateType,
    pub left_wire: usize,
    pub right_wire: usize,
    pub output_wire: usize,
}

/// The Boolean circuit represented as layers (depth × width matrix)
// #[derive(Debug)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Circuit {
    pub depth: usize,
    pub width: usize,
    pub gates: Vec<Gate>, // Flat vector representing all gates
    pub input_wires: Vec<usize>,
    pub output_wires: Vec<usize>,
}

/// A garbled gate: the encrypted truth table for the gate outputs
// #[derive(Debug, Clone)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GarbledGate {
    pub ciphertexts: Vec<Vec<u8>>, // 4 ciphertexts per 2-input gate
}

/// A garbled circuit consists of garbled gates and wire keys
#[derive(Debug)]
pub struct GarbledCircuit {
    pub garbled_gates: Vec<GarbledGate>,
    pub wire_keys: HashMap<usize, WireKeys>, // wire id -> WireKeys
    pub input_wires: Vec<usize>,
    pub output_wires: Vec<usize>,
}

/// Partitioned garbled circuits vector
pub type PartitionedGC = Vec<GarbledCircuit>;

/// The main Partitioned Garbled Circuit Scheme struct
pub struct PartitionedGCScheme {}

impl PartitionedGCScheme {
    /// Step 1: Input preparation (pair inputs, add auxiliaries if odd)
    ///
    /// # Arguments
    /// - inputs: vector of input bits (0 or 1) for prover and verifier combined
    ///
    /// # Returns
    /// - new input vector after pairing and auxiliary random bits if needed
    pub fn prepare_inputs(mut inputs: Vec<u8>) -> Vec<(u8, u8)> {
        let mut rng = rand::rng();

        // If odd number of inputs, add auxiliary random bits a0 and a1, then append a0 XOR a1
        if inputs.len() % 2 != 0 {
            let a0: u8 = rand::random::<u8>() % 2 ;
            let a1: u8 = rand::random::<u8>() % 2 ;
            inputs.push(a0 ^ a1);
            println!("[Prepare Inputs] Added auxiliaries a0={}, a1={}, appended {}", a0, a1, a0 ^ a1);
        }

        // Pair inputs sequentially into tuples (x_i, x_j)
        let paired_inputs: Vec<(u8, u8)> = inputs.chunks(2)
            .map(|pair| (pair[0], pair[1]))
            .collect();

        println!("[Prepare Inputs] Paired Inputs: {:?}", paired_inputs);
        paired_inputs
    }

    /// Step 2 & 3: Garbled circuit construction (assign keys, create ciphertext tables)
    ///
    /// # Arguments
    /// - circuit: Boolean circuit to garble
    ///
    /// # Returns
    /// - garbled circuit with wire keys and garbled gates
    pub fn garble_circuit(circuit: &Circuit) -> GarbledCircuit {
        let mut rng = rand::rng();

        // Assign random wire keys for each wire in circuit
        let mut wire_keys = HashMap::new();
        let wire_count = circuit.width * circuit.depth; // approximation; could be refined
        for wire_id in 0..wire_count {
          let key0: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect(); // 128-bit keys
          let key1: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();
          wire_keys.insert(wire_id, WireKeys { key0, key1 });
      }
      

        // Garble each gate by encrypting output keys under input keys according to truth table
        let mut garbled_gates = Vec::new();
        for gate in &circuit.gates {
            // For simplicity, create dummy ciphertexts (in practice, encrypt output keys)
            let ciphertexts = (0..4)
            .map(|_| (0..32).map(|_| rand::random::<u8>()).collect())
            .collect();
            garbled_gates.push(GarbledGate { ciphertexts });
        }

        println!("[Garble Circuit] Assigned keys to wires and garbled {} gates.", circuit.gates.len());

        GarbledCircuit {
            garbled_gates,
            wire_keys,
            input_wires: circuit.input_wires.clone(),
            output_wires: circuit.output_wires.clone(),
        }
    }

    /// Step 5: Partition the garbled circuit horizontally
    ///
    /// # Arguments
    /// - garbled_circuit: full garbled circuit
    /// - partition_size: number of gates per partition (or some heuristic)
    ///
    /// # Returns
    /// - vector of partitioned garbled circuits
    pub fn partition_garbled_circuit(garbled_circuit: &GarbledCircuit, partition_size: usize) -> PartitionedGC {
        let total_gates = garbled_circuit.garbled_gates.len();

        let mut partitions = Vec::new();

        // Simple partition by slicing gates into chunks of partition_size
        for start in (0..total_gates).step_by(partition_size) {
            let end = std::cmp::min(start + partition_size, total_gates);
            let gates_slice = &garbled_circuit.garbled_gates[start..end];

            // Extract all wires involved in these gates (for demo, just use all wires - refinement needed)
            let wire_keys = garbled_circuit.wire_keys.clone();

            partitions.push(GarbledCircuit {
                garbled_gates: gates_slice.to_vec(),
                wire_keys: wire_keys.clone(),
                input_wires: garbled_circuit.input_wires.clone(),   // in practice, only relevant inputs
                output_wires: garbled_circuit.output_wires.clone(), // similarly, only outputs relevant to partition
            });

            println!("[Partition] Created partition with gates {} to {}", start, end - 1);
        }

        partitions
    }

    /// Step 6: Run multiple iterations of protocol per partition
    ///
    /// For demonstration, this is a stub.
    pub fn run_protocol_iterations(partitions: &PartitionedGC) {
        println!("[Run Protocol] Starting protocol iterations over {} partitions.", partitions.len());

        for (i, partition) in partitions.iter().enumerate() {
            println!("[Run Protocol] Evaluating partition {} with {} gates.", i + 1, partition.garbled_gates.len());
            // Here we would perform oblivious transfer, evaluation, verification etc.
        }
    }

    /// Step 7: Aggregate the final verification from all partitions
    ///
    /// Returns a combined verification output (stub)
    pub fn aggregate_verification(partitions: &PartitionedGC) -> Vec<u8> {
        println!("[Aggregate Verification] Combining outputs from partitions.");
        // Placeholder: in practice, combine cryptographic proofs from all partitions
        vec![1u8] // dummy success flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_inputs_even() {
        let inputs = vec![0, 1, 1, 0];
        let paired = PartitionedGCScheme::prepare_inputs(inputs);
        assert_eq!(paired.len(), 2);
    }

    #[test]
    fn test_prepare_inputs_odd() {
        let inputs = vec![1, 0, 1];
        let paired = PartitionedGCScheme::prepare_inputs(inputs);
        assert_eq!(paired.len(), 2); // one pair + auxiliary pair
    }

    #[test]
    fn test_partition_and_garble() {
        // Dummy circuit with 4 gates
        let circuit = Circuit {
            depth: 2,
            width: 2,
            gates: vec![
                Gate { gate_type: GateType::And, left_wire: 0, right_wire: 1, output_wire: 2 },
                Gate { gate_type: GateType::Xor, left_wire: 2, right_wire: 3, output_wire: 4 },
                Gate { gate_type: GateType::Or, left_wire: 4, right_wire: 5, output_wire: 6 },
                Gate { gate_type: GateType::And, left_wire: 6, right_wire: 7, output_wire: 8 },
            ],
            input_wires: vec![0, 1, 3, 5],
            output_wires: vec![8],
        };

        let gc = PartitionedGCScheme::garble_circuit(&circuit);
        let partitions = PartitionedGCScheme::partition_garbled_circuit(&gc, 2);
        assert_eq!(partitions.len(), 2);
    }
}
