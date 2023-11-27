use mpz_circuits::{Circuit, BuilderError, CircuitBuilder};

use crate::enforce_one_active_bit::enforce_one_active_bit;

pub fn build_circuit() -> Result<Circuit, BuilderError> {
    let builder = CircuitBuilder::new();

    let player1_bits = builder.add_array_input::<bool, 5>();
    let player1_norm_bits = enforce_one_active_bit(&player1_bits);

    builder.add_output(player1_norm_bits);

    builder.build()
}
