/*
5th Iteration
*/

mod psg;
mod partitioned_gc;
mod public_repo;

use public_repo::publish_to_public_repo;
use crate::psg::polylithic_syntax_gen;
use crate::partitioned_gc::{Circuit, Gate, GateType, PartitionedGCScheme};

fn main() {
    // Step 1: Input logical statement (your example)
    let input = r#"The car only starts if the "start" button is pressed and the "brake" pedal is pressed"#;

    // Step 2: Generate BooleanCircuit from PSG module
    let boolean_circuit = polylithic_syntax_gen(input);
    println!("\n[Main] PSG generated boolean circuit:\n{:#?}", boolean_circuit);

    // Step 3: Convert to partitioned_gc::Circuit (hardcoded for now)
    let circuit = Circuit {
        depth: 2,
        width: 2,
        gates: vec![
            Gate { gate_type: GateType::And, left_wire: 0, right_wire: 1, output_wire: 2 },
        ],
        input_wires: vec![0, 1],
        output_wires: vec![2],
    };
    println!("\n[Main] Converted to partitioned_gc::Circuit:\n{:#?}", circuit);

    // Step 4: Prepare inputs (e.g. start and brake both pressed)
    let inputs = vec![1, 1];
    let paired_inputs = PartitionedGCScheme::prepare_inputs(inputs);

    // Step 5: Garble the circuit (returns GarbledCircuit struct)
    let garbled_circuit = PartitionedGCScheme::garble_circuit(&circuit);

    // Destructure garbled circuit into components
    let wire_keys = &garbled_circuit.wire_keys;
    let garbled_gates = &garbled_circuit.garbled_gates;

    // Step 6: Publish circuit and encrypted data to simulated DLT
    publish_to_public_repo(garbled_gates, wire_keys, &circuit).unwrap();

    // Step 7: Partition garbled circuit (1 gate per partition)
    let partitions = PartitionedGCScheme::partition_garbled_circuit(&garbled_circuit, 1);

    // Step 8: Run protocol iterations over each partition
    PartitionedGCScheme::run_protocol_iterations(&partitions);

    // Step 9: Aggregate verification result
    let verification = PartitionedGCScheme::aggregate_verification(&partitions);

    println!("\n[Main] Final verification output: {:?}", verification);
}



/*
4th Iteration
*/

// mod xor_masked_ot;

// use rand::rng;

// fn main() {
//     let seed: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();

//     let k0: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect(); // key for bit 0
//     let k1: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect(); // key for bit 1

//     // Bob chooses a bit (0 or 1)
//     let b = 1u8;

//     // Alice prepares masked keys
//     let (c0, c1) = xor_masked_ot::dummy_ot_send(&seed, &k0, &k1);

//     // Bob receives his key
//     let recovered = xor_masked_ot::dummy_ot_receive(&seed, b, &c0, &c1);

//     let expected = if b == 0 { k0 } else { k1 };
//     assert_eq!(recovered, expected);

//     println!("✅ Dummy OT works! Bob received the correct key.");
// }


/*
3rd Iteration

mod pgc;

use pgc::{Circuit, Gate, GateType, PartitionedGCScheme};

fn main() {
    // Simulated Boolean logic: (a AND b) XOR (c OR d)
    // Wire layout:
    // 0: a, 1: b, 2: c, 3: d
    // 4: a AND b
    // 5: c OR d
    // 6: (a AND b) XOR (c OR d)
    let test_circuit = Circuit {
        depth: 3,
        width: 2,
        gates: vec![
            Gate {
                gate_type: GateType::And,
                left_wire: 0,
                right_wire: 1,
                output_wire: 4,
            },
            Gate {
                gate_type: GateType::Or,
                left_wire: 2,
                right_wire: 3,
                output_wire: 5,
            },
            Gate {
                gate_type: GateType::Xor,
                left_wire: 4,
                right_wire: 5,
                output_wire: 6,
            },
        ],
        input_wires: vec![0, 1, 2, 3],
        output_wires: vec![6],
    };

    // Example binary inputs for a, b, c, d
    let inputs = vec![1, 1, 0, 1]; // Expecting output: (1 AND 1) XOR (0 OR 1) = 1 XOR 1 = 0

    // Step 1: Input preparation (pairing)
    let paired_inputs = PartitionedGCScheme::prepare_inputs(inputs);

    // Step 2–3: Garble circuit
    let garbled = PartitionedGCScheme::garble_circuit(&test_circuit);

    // Step 4: Partition circuit into blocks of 2 gates
    let partitions = PartitionedGCScheme::partition_garbled_circuit(&garbled, 2);

    // Step 5: Run protocol iterations (stubbed)
    PartitionedGCScheme::run_protocol_iterations(&partitions);

    // Step 6: Aggregate output
    let final_verification = PartitionedGCScheme::aggregate_verification(&partitions);

    println!("\nFinal Verification Result: {:?}", final_verification);
}
*/

/*
2nd Iteration
*/
// mod psg;
// mod partitioned_gc;
// mod public_repo;
// use public_repo::publish_to_public_repo;


// use crate::psg::polylithic_syntax_gen;
// use crate::partitioned_gc::{Circuit, Gate, GateType, PartitionedGCScheme};

// fn main() {
//     // Step 1: Input logical statement (your example)
//     let input = r#"The car only starts if the "start" button is pressed and the "brake" pedal is pressed"#;

//     // Step 2: Generate BooleanCircuit from PSG module
//     let boolean_circuit = polylithic_syntax_gen(input);

//     println!("\n[Main] PSG generated boolean circuit:\n{:#?}", boolean_circuit);

//     // Step 3: Convert PSG BooleanCircuit to partitioned_gc::Circuit for garbling
//     //
//     // For demonstration, we create a dummy circuit with simple gates
//     // based on the number of gates in PSG output (usually you'd parse PSG output precisely)
//     let circuit = Circuit {
//         depth: 2,
//         width: 2,
//         gates: vec![
//             Gate { gate_type: GateType::And, left_wire: 0, right_wire: 1, output_wire: 2 },
//             // Add more gates here if needed, matching boolean_circuit.gates.len()
//         ],
//         input_wires: vec![0, 1],
//         output_wires: vec![2],
//     };

//     println!("\n[Main] Converted to partitioned_gc::Circuit:\n{:#?}", circuit);

//     // Step 4: Prepare inputs for partitioned GC protocol (example inputs)
//     let inputs = vec![1, 1]; // start button pressed = 1, brake pressed = 1
//     let paired_inputs = PartitionedGCScheme::prepare_inputs(inputs);

//     // Step 5: Garble circuit
//     let garbled_circuit = PartitionedGCScheme::garble_circuit(&circuit);

//     // Step 6: Partition the garbled circuit (set partition size as 1 gate for demonstration)
//     let partitions = PartitionedGCScheme::partition_garbled_circuit(&garbled_circuit, 1);

//     publish_to_public_repo(&garbled_gates, &wire_keys, &circuit).unwrap();


//     // Step 7: Run protocol iterations over partitions (stub)
//     PartitionedGCScheme::run_protocol_iterations(&partitions);

//     // Step 8: Aggregate verification result (stub)
//     let verification = PartitionedGCScheme::aggregate_verification(&partitions);

//     println!("\n[Main] Final verification output: {:?}", verification);
// }


/*
1st Iteration
*/
// mod psg; // import the psg module

// use crate::psg::polylithic_syntax_gen;

// fn main() {
//     let input = r#"The car only starts if the "start" button is pressed and the "brake" pedal is pressed"#;

//     let circuit = polylithic_syntax_gen(input);

//     println!("Generated Boolean Circuit:\n{:#?}", circuit);
// }
