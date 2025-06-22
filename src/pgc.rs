use rand::{RngCore, rngs::OsRng};
use hex::encode as hex_encode;

type Key = Vec<u8>;
type CipherText = Vec<u8>;

#[derive(Debug, Clone)]
struct Gate {
    left_wire: usize,
    right_wire: usize,
    output_wire: usize,
    gate_type: String, // "AND", "XOR", etc.
    ciphertexts: Vec<CipherText>, // 4 ciphertexts per gate
}

#[derive(Debug)]
struct Circuit {
    gates: Vec<Gate>,
    num_inputs: usize,
    num_wires: usize,
}

fn generate_random_key() -> Key {
    let mut key = vec![0u8; 16];
    OsRng.fill_bytes(&mut key);
    key
}

// Step 1: Preparation - pad inputs if odd number
fn prepare_inputs(mut inputs: Vec<bool>) -> Vec<bool> {
    println!("--- Step 1: Preparation ---");
    println!("[Input] Original inputs: {:?}", inputs);
    if inputs.len() % 2 != 0 {
        println!("[Preparation] Odd number of inputs, adding two auxiliary bits a0=0, a1=1");
        inputs.push(false); // a0 = 0
        inputs.push(true);  // a1 = 1
    }
    println!("[Preparation] Inputs after padding: {:?}", inputs);
    inputs
}

// Step 2: Assign keys for each wire (input + internal)
fn assign_keys(num_wires: usize) -> Vec<(Key, Key)> {
    println!("--- Step 2: Assigning Wire Keys ---");
    let mut wire_keys = Vec::with_capacity(num_wires);
    for w in 0..num_wires {
        let k0 = generate_random_key();
        let k1 = generate_random_key();
        println!("[AssignWireKeys] Wire {}: k0 = {}, k1 = {}", w, hex_encode(&k0), hex_encode(&k1));
        wire_keys.push((k0, k1));
    }
    wire_keys
}

// Step 3: Garble gates with dummy ciphertexts (4 per gate)
fn garble_gates(circuit: &mut Circuit, wire_keys: &[(Key, Key)]) {
    println!("--- Step 3: Garbling Gates ---");
    for (gate_idx, gate) in circuit.gates.iter_mut().enumerate() {
        gate.ciphertexts.clear();
        for ct_idx in 0..4 {
            let ct = generate_random_key(); // dummy ciphertext as random key
            println!("  [GarbleGate] Gate {} Ciphertext {}: {}", gate_idx, ct_idx, hex_encode(&ct));
            gate.ciphertexts.push(ct);
        }
    }
}

// Step 4: Partition circuit horizontally (n/1 ratio, leftmost input gates per partition)
fn partition_circuit(circuit: &Circuit, partition_size: usize) -> Vec<Circuit> {
    println!("--- Step 4: Partitioning Circuit ---");
    let total_gates = circuit.gates.len();
    println!("[Partition] Total gates: {}", total_gates);
    let mut partitions = Vec::new();

    let mut start = 0;
    while start < total_gates {
        let end = usize::min(start + partition_size, total_gates);
        let gates_part = circuit.gates[start..end].to_vec();

        let part_circuit = Circuit {
            gates: gates_part,
            num_inputs: circuit.num_inputs,
            num_wires: circuit.num_wires,
        };

        println!("[Partition] Created partition with gates {} to {}", start, end - 1);
        partitions.push(part_circuit);
        start = end;
    }
    println!("[Partition] Total partitions created: {}", partitions.len());
    partitions
}

// Step 5: Dummy OT Verification per partition (simulated)
fn ot_verification(partitions: &[Circuit]) -> Vec<bool> {
    println!("--- Step 5: OT Verification per Partition ---");
    let mut results = Vec::new();
    for (i, part) in partitions.iter().enumerate() {
        println!("[OT Verification] Running OT on partition {} with {} gates...", i, part.gates.len());
        // Simulated verification logic (always succeeds)
        println!("[OT Verification] Partition {} verification successful.", i);
        results.push(true);
    }
    results
}

// Step 6: Aggregate results from OT verification
fn aggregate_results(results: &[bool]) -> bool {
    println!("--- Step 6: Aggregating OT Verification Results ---");
    let final_result = results.iter().all(|&r| r);
    println!("[Aggregation] Final combined verification output: {}", final_result);
    final_result
}

// Main Driver function simulating entire flow
fn run_partitioned_garbled_circuit_demo() {
    println!("=== Partitioned Garbled Circuit Generation Scheme Demo ===");

    // Sample boolean inputs (prover + verifier)
    let original_inputs = vec![true, false, true]; // Odd number to test auxiliary padding

    // Step 1: Prepare inputs (pad if odd)
    let prepared_inputs = prepare_inputs(original_inputs);

    // Create a simple dummy circuit for demo (say 4 gates)
    // Each gate uses two input wires, outputs to a new wire
    // Wire indexing: 0..inputs, then internal wires continue from num_inputs
    let num_inputs = prepared_inputs.len();
    let num_wires = num_inputs + 4; // extra wires for outputs
    let mut circuit = Circuit {
        gates: vec![
            Gate { left_wire: 0, right_wire: 1, output_wire: num_inputs, gate_type: "AND".to_string(), ciphertexts: vec![] },
            Gate { left_wire: 2, right_wire: 3, output_wire: num_inputs + 1, gate_type: "XOR".to_string(), ciphertexts: vec![] },
            Gate { left_wire: num_inputs, right_wire: num_inputs + 1, output_wire: num_inputs + 2, gate_type: "OR".to_string(), ciphertexts: vec![] },
            Gate { left_wire: num_inputs + 2, right_wire: 4, output_wire: num_inputs + 3, gate_type: "AND".to_string(), ciphertexts: vec![] },
        ],
        num_inputs,
        num_wires,
    };

    println!("[Circuit] Created circuit with {} gates and {} wires", circuit.gates.len(), circuit.num_wires);

    // Step 2: Assign keys to wires
    let wire_keys = assign_keys(circuit.num_wires);

    // Step 3: Garble gates with ciphertexts
    garble_gates(&mut circuit, &wire_keys);

    // Step 4: Partition circuit into smaller circuits (e.g., size 2)
    let partitions = partition_circuit(&circuit, 2);

    // Step 5: Run OT verification per partition
    let ot_results = ot_verification(&partitions);

    // Step 6: Aggregate OT results for final verification
    let final_verification = aggregate_results(&ot_results);

    println!("=== Partitioned Garbled Circuit Generation Complete ===");
    println!("Final verification status: {}", final_verification);
}

fn main() {
    run_partitioned_garbled_circuit_demo();
}
