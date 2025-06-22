# Module II: Partitioned Garbled Circuits

> Reference: Yao's Garbled Circuits basics
> Partitioning scheme for splitting circuits into multiple independent garbled circuits
> Ensures privacy and correctness via cryptographic commitments

## Goal

Securely divide the table into multiple representation of the truth table matrix `T`.

## Understanding the Concept

### 1. Boolean Circuit Representation

The Boolean circuit can be represented as a Directed Acyclic Graph (DAG) with fixed 2-input gates (left `l` and right `r` inputs).

The circuit has depth d (number of layers) and width n (number of gates per layer).

Representing it as a matrix `M` of dimension `d × n` where each element is a gate as it is convinient in defining the composition of layers.

### 2. Partitioned Garbled Circuits (PGCs)

The circuit `C` can be partitioned into smaller sub-circuits `(C1, C2, ...)` each independently garbled.

**This allows multi-party OT verification protocols to run verification and privacy checks independently on parts of the circuit.**

_The garbled truth tables (or garbled gate tables) are split accordingly._

### 3. Correctness

> To obtain the security properties of such scheme,
> it can be proved with [state-separating proofs](https://eprint.iacr.org/2021/1453.pdf) and
> to make cryptographic proofs more suitable for multi-parties verification.

For security parameter `k`, garbling and evaluation satisfy:

- `∀(C,e,d)` in garbling scheme `Gc(1^k,C)` and input `x`,
- `De(d, Ev(C, En(e,x)))` = `C(x)`,
  meaning, _decoding the evaluation result yields the correct circuit output._

## Input

| Input to Partitioned Garbled Circuit Scheme | Description                                      |
| ------------------------------------------- | ------------------------------------------------ |
| Circuit `C`                                 | Boolean circuit structure (DAG, matrix of gates) |
| Input bits `x_1, ..., x_n`                  | Prover and verifier inputs bits                  |
| Auxiliary bits `a_0, a_1` (if needed)       | Random padding bits if odd inputs                |
| Truth tables `T` per gate                   | Defines gate logic for garbling                  |

## To do

- You pass the Boolean circuit 𝐶 , C as a data structure (list of gates with their inputs and outputs)
- The input values 𝑥 , x as bit arrays or boolean vectors matching input wires

The scheme uses these to:

- Pair inputs into 2-input gates
- Assign wire keys
- Generate garbled tables per gate
- Partition the circuit into smaller garbled circuits
- Run oblivious transfers on each partitioned circuit

## Result

#### main.rs

add the below snippet in main,

```
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
```

![Partitioned Garbled Circuit](assets/PartitionedGarbledCircuit.png)

**Output**

```
--- Polylithic Syntax Generation Complete ---

[Main] PSG generated boolean circuit:
BooleanCircuit {
    gates: [
        "circuit((cced28c6dc3f99c2396a5eaad732bf6b28142335892b1cd0e6af6cdb53f5ccfa) && (876797249822df52fd57ad8a506093acec21dd5967cad35984af03762560b906))",
    ],
}

[Main] Converted to partitioned_gc::Circuit:
Circuit {
    depth: 2,
    width: 2,
    gates: [
        Gate {
            gate_type: And,
            left_wire: 0,
            right_wire: 1,
            output_wire: 2,
        },
    ],
    input_wires: [
        0,
        1,
    ],
    output_wires: [
        2,
    ],
}
[Prepare Inputs] Paired Inputs: [(1, 1)]
[Garble Circuit] Assigned keys to wires and garbled 1 gates.
[Partition] Created partition with gates 0 to 0
[Run Protocol] Starting protocol iterations over 1 partitions.
[Run Protocol] Evaluating partition 1 with 1 gates.
[Aggregate Verification] Combining outputs from partitions.

[Main] Final verification output: [1]
```
