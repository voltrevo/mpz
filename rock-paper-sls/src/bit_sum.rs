use crate::tree_reduce::tree_reduce;
use mpz_circuits::{Tracer, types::Bit};

pub fn bit_sum<'a>(bits: &[Tracer<'a, Bit>]) -> BitAddState<'a> {
    let add_states = bits.iter().map(|bit| BitAddState::Bit(*bit)).collect::<Vec<_>>();
    tree_reduce(&add_states, &|a, b| a.add(b))
}

#[derive(Clone)]
pub enum BitAddState<'a> {
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
