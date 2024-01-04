#[derive(Drop)]
struct InputData {
    a: felt252,
    b: felt252,
    c: Array<felt252>,
}

fn main(x: InputData) -> (felt252, felt252, felt252, felt252) {
    f(x)
}

fn f(x: InputData) -> (felt252, felt252, felt252, felt252) {
    let d_len = x.c.span().len();
    let mut sum_d = 0_felt252;
    let mut i: u32 = 0;
    loop {
        if i == d_len {
            break;
        }
        sum_d = sum_d + *x.c.at(i);
        i += 1;
    };
    (x.a, x.b, d_len.into(), sum_d)
}

#[cfg(test)]
mod tests {
    use super::{f, InputData};

    #[test]
    fn it_works() {
        assert(f(InputData { a: 1, b: 2, c: array![1, 2, 3] }) == (1, 2, 3, 6), 'it works!');
    }
}
