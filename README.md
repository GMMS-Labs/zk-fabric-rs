# zk-fabric-rs

a Polylithic Syntax Zero Knowledge Joint Proof System

## Module 1: Polylithic Syntax Generation

Everything about it, what is Polylithic? How is it generation? pros, goals~, see the [PSG Wiki](wiki/psg.md).

## Module 2: Partioned Garbled Circuits

Understanding the concept, see the [PGC Wiki](wiki/pgc.md).

### Status So Far...

```
- PSG generates the boolean circuit with hashed variables and operators.

- The circuit is converted into your partitioned_gc::Circuit structure.

- Inputs get paired and prepared properly.

- Garbling assigns keys and garbles the gates correctly.

- Partitioning splits the circuit as expected.

- Protocol runs over the partitions and evaluates them.

- Outputs from partitions are aggregated, resulting in final verification [1] (true).
```

