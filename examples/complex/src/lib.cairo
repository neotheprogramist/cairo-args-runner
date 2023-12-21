fn main(x: felt252, y: felt252, z: felt252, a: Array<felt252>) -> (felt252, felt252) {
    complex(x, y, z, a)
}

fn complex(x: felt252, y: felt252, z: felt252, a: Array<felt252>) -> (felt252, felt252) {
    let mut sum = 0_felt252;
    let mut i: u32 = 0;
    loop {
        if i == a.len() {
            break;
        }
        sum = sum + *a.at(i);
        i += 1;
    };
    let mut i: u32 = 0;
    (x * y - z, sum)
}

#[cfg(test)]
mod tests {
    use super::complex;

    #[test]
    fn it_works() {
        assert(complex(10, 2, 20, array![1, 2, 4, 8, 16]) == (0, 31), 'it works!');
    }
}
