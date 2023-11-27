use mpz_circuits::{types::Bit, BuilderError, Circuit, CircuitBuilder, Tracer};

use crate::{
    enforce_one_active_bit::enforce_one_active_bit,
    game::{GameAction, GameResult},
    tree_reduce::tree_reduce,
};

pub fn build_circuit() -> Result<Circuit, BuilderError> {
    let builder = CircuitBuilder::new();

    let player1_bits = builder.add_array_input::<bool, 5>();
    let player1_norm_bits = enforce_one_active_bit(&player1_bits);

    let player2_bits = builder.add_array_input::<bool, 5>();
    let player2_norm_bits = enforce_one_active_bit(&player2_bits);

    let mut player1_win_conditions = Vec::<Tracer<Bit>>::new();
    let mut draw_conditions = Vec::<Tracer<Bit>>::new();

    for i in 0..5 {
        let player1_bit = player1_norm_bits[i];
        let player1_action = GameAction::from(i).unwrap();

        for j in 0..5 {
            let player2_bit = player2_norm_bits[j];
            let player2_action = GameAction::from(j).unwrap();

            match GameResult::from(player1_action, player2_action) {
                GameResult::Player1 => player1_win_conditions.push(player1_bit & player2_bit),
                GameResult::Player2 => {}
                GameResult::Draw => draw_conditions.push(player1_bit & player2_bit),
            }
        }
    }

    let player1_wins = tree_reduce(&player1_win_conditions, &|a, b| *a ^ *b);
    let draw = tree_reduce(&draw_conditions, &|a, b| *a ^ *b);
    let player2_wins = !player1_wins & !draw;

    builder.add_output(vec![player1_wins, player2_wins, draw]);

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_circuit() {
        let circuit = build_circuit().expect("failed to build circuit");

        for i in 0..5 {
            let player1_action = GameAction::from(i).unwrap();

            for j in 0..5 {
                let player2_action = GameAction::from(j).unwrap();
                let expected_result = GameResult::from(player1_action, player2_action);

                assert_eq!(
                    circuit
                        .evaluate(&[player1_action.into(), player2_action.into()])
                        .expect("failed to evaluate circuit"),
                    vec![expected_result.into()]
                );
            }
        }
    }
}
