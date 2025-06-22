# to store

## Garbled Circuit Structure

JSON (or binary) file containing all gates and their ciphertexts.

## Wire Keys Table (Encrypted)

For each wire, store key0, key1. Optionally encrypt or obfuscate later.

## Metadata

Input/output wire indices, maybe a manifest or circuit hash.

# structure

public_repo/
├── circuit.json # Garbled gates and structure
├── wire_keys.json # Wire keys (could be public or encrypted)
└── manifest.json # Metadata: wire mappings, hash, etc.

# result

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

[Public Repo] Published garbled circuit to public_repo/published_circuit.json
[Partition] Created partition with gates 0 to 0
[Run Protocol] Starting protocol iterations over 1 partitions.
[Run Protocol] Evaluating partition 1 with 1 gates.
[Aggregate Verification] Combining outputs from partitions.

[Main] Final verification output: [1]
```
