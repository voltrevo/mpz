mod bit_sum;
mod build_circuit;
mod enforce_one_active_bit;
mod game;
mod tree_reduce;

use build_circuit::build_circuit;
use mpz_circuits::types::Value;

use crate::game::GameAction;

fn main() {
    let circuit = build_circuit().expect("failed to build circuit");

    let player1_action: Value = GameAction::Rock.into();
    let player2_action: Value = GameAction::Scissors.into();

    println!("Player 1:");
    print_action(&player1_action);

    println!("Player 2:");
    print_action(&player2_action);

    let result = circuit
        .evaluate(&[player1_action, player2_action])
        .expect("failed to evaluate circuit");

    println!("Result:");
    print_result(&result[0]);
}

fn print_action(action: &Value) {
    let bits = match action {
        Value::Array(bits) => bits,
        _ => panic!("expected array"),
    };

    let bits = bits
        .iter()
        .map(|bit| match bit {
            Value::Bit(b) => *b,
            _ => panic!("expected bit"),
        })
        .collect::<Vec<_>>();

    println!("  Rock:      {}", bits[0] as usize);
    println!("  Paper:     {}", bits[1] as usize);
    println!("  Scissors:  {}", bits[2] as usize);
    println!("  Lizard:    {}", bits[3] as usize);
    println!("  Spock:     {}", bits[4] as usize);
}

fn print_result(result: &Value) {
    let bits = match result {
        Value::Array(bits) => bits,
        _ => panic!("expected array"),
    };

    let bits = bits
        .iter()
        .map(|bit| match bit {
            Value::Bit(b) => *b,
            _ => panic!("expected bit"),
        })
        .collect::<Vec<_>>();

    println!("  Player 1:  {}", bits[0] as usize);
    println!("  Player 2:  {}", bits[1] as usize);
    println!("  Draw:      {}", bits[2] as usize);
}
