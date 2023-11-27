use mpz_circuits::{types::Bit, Tracer};

use crate::bit_sum::{bit_sum, BitAddState};

pub fn enforce_one_active_bit<'a>(bits: &[Tracer<'a, Bit>]) -> Vec<Tracer<'a, Bit>> {
    let sum = bit_sum(bits);

    let valid = match sum {
        BitAddState::Bit(_) => panic!("sum should be compound"),
        BitAddState::Compound { count, overflow } => {
            let overflow = overflow ^ count;
            let overflow = overflow ^ overflow;
            !overflow & count
        }
    };

    bits.iter()
        .enumerate()
        .map(|(i, bit)| match i {
            0 => !valid | *bit,
            _ => valid & *bit,
        })
        .collect()
}

#[cfg(test)]
mod test {
    use mpz_circuits::{types::Value, CircuitBuilder};

    use crate::enforce_one_active_bit::enforce_one_active_bit;

    #[test]
    fn test_enforce_one_active_bit() {
        let builder = CircuitBuilder::new();

        let bits = builder.add_array_input::<bool, 5>();
        let norm_bits = enforce_one_active_bit(&bits);

        builder.add_output(norm_bits);

        let circuit = builder.build().unwrap();

        // When one bit is set, keep that bit
        for i in 0..5 {
            let mut bits = vec![Value::Bit(false); 5];
            bits[i] = Value::Bit(true);

            assert_eq!(
                &circuit
                    .evaluate(&[Value::Array(bits.clone())])
                    .expect("failed to evaluate circuit"),
                &vec![Value::Array(bits)]
            );
        }

        // When all bits are unset, pick the first bit
        assert_eq!(
            circuit
                .evaluate(&[Value::Array(vec![
                    Value::Bit(false),
                    Value::Bit(false),
                    Value::Bit(false),
                    Value::Bit(false),
                    Value::Bit(false),
                ])])
                .expect("failed to evaluate circuit"),
            &[Value::Array(vec![
                Value::Bit(true),
                Value::Bit(false),
                Value::Bit(false),
                Value::Bit(false),
                Value::Bit(false),
            ])]
        );

        // When multiple bits are set, pick the first bit
        assert_eq!(
            circuit
                .evaluate(&[Value::Array(vec![
                    Value::Bit(false),
                    Value::Bit(false),
                    Value::Bit(true),
                    Value::Bit(false),
                    Value::Bit(true),
                ])])
                .expect("failed to evaluate circuit"),
            &[Value::Array(vec![
                Value::Bit(true),
                Value::Bit(false),
                Value::Bit(false),
                Value::Bit(false),
                Value::Bit(false),
            ])]
        );
    }
}
