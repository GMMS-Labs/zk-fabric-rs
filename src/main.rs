/*
2nd Iteration
*/
mod psg;
mod partitioned_gc;

use crate::psg::polylithic_syntax_gen;
use crate::partitioned_gc::{Circuit, Gate, GateType, PartitionedGCScheme};

fn main() {
    // Step 1: Input logical statement (your example)
    let input = r#"The car only starts if the "start" button is pressed and the "brake" pedal is pressed"#;

    // Step 2: Generate BooleanCircuit from PSG module
    let boolean_circuit = polylithic_syntax_gen(input);

    println!("\n[Main] PSG generated boolean circuit:\n{:#?}", boolean_circuit);

    // Step 3: Convert PSG BooleanCircuit to partitioned_gc::Circuit for garbling
    //
    // For demonstration, we create a dummy circuit with simple gates
    // based on the number of gates in PSG output (usually you'd parse PSG output precisely)
    let circuit = Circuit {
        depth: 2,
        width: 2,
        gates: vec![
            Gate { gate_type: GateType::And, left_wire: 0, right_wire: 1, output_wire: 2 },
            // Add more gates here if needed, matching boolean_circuit.gates.len()
        ],
        input_wires: vec![0, 1],
        output_wires: vec![2],
    };

    println!("\n[Main] Converted to partitioned_gc::Circuit:\n{:#?}", circuit);

    // Step 4: Prepare inputs for partitioned GC protocol (example inputs)
    let inputs = vec![1, 1]; // start button pressed = 1, brake pressed = 1
    let paired_inputs = PartitionedGCScheme::prepare_inputs(inputs);

    // Step 5: Garble circuit
    let garbled_circuit = PartitionedGCScheme::garble_circuit(&circuit);

    // Step 6: Partition the garbled circuit (set partition size as 1 gate for demonstration)
    let partitions = PartitionedGCScheme::partition_garbled_circuit(&garbled_circuit, 1);

    // Step 7: Run protocol iterations over partitions (stub)
    PartitionedGCScheme::run_protocol_iterations(&partitions);

    // Step 8: Aggregate verification result (stub)
    let verification = PartitionedGCScheme::aggregate_verification(&partitions);

    println!("\n[Main] Final verification output: {:?}", verification);
}


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
