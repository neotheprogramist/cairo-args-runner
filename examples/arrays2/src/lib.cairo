fn main(a: Array<felt252>, b: Array<felt252>) -> (felt252, felt252, felt252, felt252) {
    complex(a, b)
}

fn complex(a: Array<felt252>, b: Array<felt252>) -> (felt252, felt252, felt252, felt252) {
    let a_len = a.span().len();
    let b_len = b.span().len();

    let mut sum_a = 0_felt252;
    let mut i: u32 = 0;
    loop {
        if i == a_len {
            break;
        }
        sum_a = sum_a + *a.span().at(i);
        i += 1;
    };

    let mut sum_b = 0_felt252;
    let mut i: u32 = 0;
    loop {
        if i == b_len {
            break;
        }
        sum_b = sum_b + *b.at(i);
        i += 1;
    };

    (a_len.into(), sum_a, b_len.into(), sum_b)
}

#[cfg(test)]
mod tests {
    use super::complex;

    #[test]
    fn it_works() {
        assert(
            complex(array![1, 1, 1, 1, 1], array![2, 2, 2, 2, 2, 2]) == (5, 5, 6, 12), 'it works!'
        );
    }
}
