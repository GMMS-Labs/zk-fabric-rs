/*
# Polylithic Syntax Generation
Example of a composite statement:
`The car only starts [if] the "start" button [and] the brake pedal is pressed.`
where [.] represents logical relationship between variables (the rest of the words in composite statement).

This represents a logical condition where:
  S: Car starts
  P: "start" button is pressed
  B: brake pedal is pressed

It can be logically modeled as:
  (P ∧ B) → S  or  S → (P ∧ B)
depending on direction of implication.

Extracting [quoted strings] as variables and logic keywords like [and] as Boolean operators.
*/ 

// use std::collections::HashMap;
use sha2::{Sha256, Digest};

/// Logical variable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable(String);

/// Logical operator enum
#[derive(Debug, Clone)]
pub enum Operator {
    And,
    Or,
    Xor,
    // Not,
    // Extendable
}

/// Internal representation of a parsed logical rule
#[derive(Debug)]
pub struct LogicExpression {
    variables: Vec<String>,
    operators: Vec<Operator>,
}

/// Boolean circuit representation (placeholder)
#[derive(Debug)]
pub struct BooleanCircuit {
    gates: Vec<String>, // Could be a graph or logic gate tree
}

/// === Step 1: Extractor_v ===
fn extractor_v(input: &str) -> Vec<Variable> {
  let vars: Vec<Variable> = input
  .split_whitespace()
  .filter(|s| s.starts_with('"') && s.ends_with('"'))
  .map(|s| Variable(s.trim_matches('"').to_string()))
  .collect();
  println!("[Extractor_v] Extracted Variables: {:?}", vars);
  vars    
}

/// === Step 2: Extractor_o ===
fn extractor_o(input: &str) -> Vec<Operator> {
  let mut ops = Vec::new();
  if input.contains("and") {
      ops.push(Operator::And);
      println!("[Extractor_o] Found Operator: AND");
  }
  if input.contains("or") {
      ops.push(Operator::Or);
      println!("[Extractor_o] Found Operator: OR");
  }
  if input.contains("xor") {
      ops.push(Operator::Xor);
      println!("[Extractor_o] Found Operator: XOR");
  }
  ops
}

/// === Step 3: Hash variables ===
fn hash_variables(vars: &[Variable]) -> Vec<String> {
  let hashed: Vec<String> = vars.iter().map(|var| {
      let mut hasher = Sha256::new();
      hasher.update(var.0.as_bytes());
      let result = format!("{:x}", hasher.finalize());
      println!("[Hash] '{}' => {}", var.0, result);
      result
  }).collect();
  println!("[HashVariables](To hide actual variable values) Hashed Variables: {:?}", hashed);
  hashed
}

/// === Step 4: Regexp(variables, ops) ===
fn build_regexp(vars: &[String], ops: &[Operator]) -> String {
  let mut pattern = String::new();
  for (i, var) in vars.iter().enumerate() {
      pattern.push_str(&format!("({})", var));
      if let Some(op) = ops.get(i) {
          let sym = match op {
              Operator::And => "&&",
              Operator::Or => "||",
              Operator::Xor => "^",
              // Operator::Not => "!",
          };
          pattern.push_str(&format!(" {} ", sym));
      }
  }
  println!("[Regexp](Form a regular expression from variable and ops) Built Expression: {}", pattern);
  pattern
}

/// === Step 5: CircuitGen ===
fn circuit_gen(expression: &str) -> BooleanCircuit {
  println!("[CircuitGen](Compile the regular expression into boolean circuits) Generating circuit for: {}", expression);
  BooleanCircuit {
      gates: vec![format!("circuit({})", expression)],
  }
}

/// === Step 6: K-map Optimization ===
fn karnaugh_optimize(circuit: BooleanCircuit) -> BooleanCircuit {
  println!("[KarnaughOptimize] Optimizing circuit (placeholder)");
  circuit
}

/// === Main Driver ===
pub fn polylithic_syntax_gen(input: &str) -> crate::psg::BooleanCircuit {
  println!("--- Polylithic Syntax Generation Start ---");
  println!("[Input] Composite Statement: {}", input);

  let vars = extractor_v(input);
  let ops = extractor_o(input);
  let hashed_vars = hash_variables(&vars);
  let regexp_expr = build_regexp(&hashed_vars, &ops);
  let raw_circuit = circuit_gen(&regexp_expr);
  let final_circuit = karnaugh_optimize(raw_circuit);

  println!("--- Polylithic Syntax Generation Complete ---");
  final_circuit
}