fn main(
    a: Array<felt252>, b: Array<felt252>
) -> (felt252, felt252) {
    complex(a, b)
}

fn complex(
    a: Array<felt252>, b: Array<felt252>
) -> (felt252, felt252) {
    let mut sum = 0_felt252;
    let mut i: u32 = 0;
    loop {
        if i == a.len() {
            break;
        }
        sum = sum + *a.at(i);
        i += 1;
    };
    let mut sum2 = 0_felt252;
    let mut i: u32 = 0;
    loop {
        if i == b.len() {
            break;
        }
        sum = sum + *b.at(i);
        i += 1;
    };
    let mut i: u32 = 0;
    (sum, sum2)
}

#[cfg(test)]
mod tests {
    use super::complex;

    #[test]
    fn it_works() {
        assert(complex(array![1, 2, 4, 8, 16], array![]) == (31, 0), 'it works!');
    }
}
