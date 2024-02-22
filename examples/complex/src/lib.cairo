use core::traits::Into;
use core::array::SpanTrait;
use core::array::ArrayTrait;

#[derive(Drop, Serde)]
struct InputData {
    a: Array<felt252>,
    b: Array<felt252>,
}

fn main(x: InputData) -> (felt252, felt252, felt252, felt252) {
    complex(x.a, x.b)
}

fn complex(a: Array<felt252>, b: Array<felt252>) -> (felt252, felt252, felt252, felt252) {
    let a_len = a.span().len();
    let b_len = b.span().len();
    let mut sum = 0_felt252;
    let mut i: u32 = 0;
    loop {
        if i == a_len {
            break;
        }
        sum = sum + *a.span().at(i);
        i += 1;
    };
    let mut sum2 = 0_felt252;
    let mut i: u32 = 0;
    loop {
        if i == b_len {
            break;
        }
        sum2 = sum2 + *b.at(i);
        i += 1;
    };
    (sum, sum2, a_len.into(), b_len.into())
}

#[cfg(test)]
mod tests {
    use super::complex;

    #[test]
    fn it_works() {
        assert(
            complex(array![1, 2, 4, 8, 16], array![1, 2, 3, 4, 5, 6]) == (31, 21, 5, 6), 'it works!'
        );
    }
}
