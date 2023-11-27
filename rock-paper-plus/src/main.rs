mod build_circuit;
mod enforce_one_active_bit;
mod tree_reduce;
mod bit_sum;

use mpz_circuits::types::Value;
use build_circuit::build_circuit;

fn main() {
    let circuit = build_circuit().expect("failed to build circuit");

    let output = circuit.evaluate(&[Value::Array(vec![
        Value::Bit(false),
        Value::Bit(false),
        Value::Bit(false),
        Value::Bit(false),
        Value::Bit(false),
    ])]).expect("failed to evaluate circuit");

    dbg!(output);
}
