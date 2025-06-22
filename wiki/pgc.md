# Module II: Partitioned Garbled Circuits

## Understanding the Concept

### 1. Boolean Circuit Representation

The Boolean circuit is represented as a Directed Acyclic Graph (DAG) with fixed 2-input gates (left `l` and right `r` inputs).

The circuit has depth d (number of layers) and width n (number of gates per layer).

We can represent it as a matrix `M` of dimension `d × n` where each element is a gate.

### 2. Partitioned Garbled Circuits (PGCs)

The circuit `C` can be partitioned into smaller sub-circuits `(C1, C2, ...)` each independently garbled.

**This allows multi-party OT verification protocols to run verification and privacy checks independently on parts of the circuit.**

The garbled truth tables (or garbled gate tables) are split accordingly.

### 3. Correctness

For security parameter `k`, garbling and evaluation satisfy:
`∀(C,e,d)` in garbling scheme `Gc(1^k,C)` and input `x`,
`De(d, Ev(C, En(e,x)))` = `C(x)`,
_meaning decoding the evaluation result yields the correct circuit output._
