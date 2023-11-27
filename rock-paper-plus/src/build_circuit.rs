use mpz_circuits::{Circuit, BuilderError, CircuitBuilder, types::Bit, Tracer};

pub fn build_circuit() -> Result<Circuit, BuilderError> {
    let builder = CircuitBuilder::new();

    let player1_bits = builder.add_array_input::<bool, 5>();
    let player1_norm_bits = enforce_one_active_bit(&player1_bits);

    builder.add_output(player1_norm_bits);

    builder.build()
}

fn enforce_one_active_bit<'a>(bits: &[Tracer<'a, Bit>]) -> Vec<Tracer<'a, Bit>> {
    let add_states = bits.iter().map(|bit| BitAddState::Bit(*bit)).collect::<Vec<_>>();

    let sum = tree_reduce(&add_states, &|a, b| a.add(b));

    let valid = match sum {
        BitAddState::Bit(_) => panic!("sum should be compound"),
        BitAddState::Compound { count, overflow } => {
            let overflow = overflow ^ count;
            let overflow = overflow ^ overflow;
            !overflow & count
        }
    };

    bits.iter().enumerate().map(|(i, bit)| {
        match i {
            0 => !valid | *bit,
            _ => valid & *bit,
        }
    }).collect()
}

fn tree_reduce<T: Clone>(values: &[T], combine: &impl Fn(&T, &T) -> T) -> T {
    assert!(!values.is_empty());

    if values.len() == 1 {
        values[0].clone()
    } else {
        let mid = values.len() / 2;
        let (left, right) = values.split_at(mid);
        let left = tree_reduce(left, combine);
        let right = tree_reduce(right, combine);
        combine(&left, &right)
    }
}

#[derive(Clone)]
enum BitAddState<'a> {
    Bit(Tracer<'a, Bit>),
    Compound {
        count: Tracer<'a, Bit>,
        overflow: Tracer<'a, Bit>,
    },
}

impl<'a> BitAddState<'a> {
    fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (BitAddState::Bit(a), BitAddState::Bit(b)) => BitAddState::Compound {
                count: *a ^ *b,
                overflow: *a & *b,
            },
            (BitAddState::Bit(a), BitAddState::Compound { count, overflow }) => {
                BitAddState::Compound {
                    count: *a ^ *count,
                    overflow: *overflow | (*a & *count),
                }
            }
            (BitAddState::Compound { count, overflow }, BitAddState::Bit(b)) => {
                BitAddState::Compound {
                    count: *count ^ *b,
                    overflow: *overflow | (*count & *b),
                }
            }
            (BitAddState::Compound { count: a_count, overflow: a_overflow }, BitAddState::Compound { count: b_count, overflow: b_overflow }) => {
                BitAddState::Compound {
                    count: *a_count ^ *b_count,
                    overflow: *a_overflow | *b_overflow | (*a_count & *b_count),
                }
            }
        }
    }
}
