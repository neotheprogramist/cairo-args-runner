use core::traits::Into;
use core::array::SpanTrait;
use core::array::ArrayTrait;
fn main(a: Array<felt252>, b: Array<felt252>) -> (felt252, felt252, felt252, felt252) {
    complex(a, b)
}

fn complex(a: Array<felt252>, b: Array<felt252>) -> (felt252, felt252, felt252, felt252) {
    let mut sum = 0_felt252;
    let mut i: u32 = 0;
    let a_len = a.span().len();
    let b_len = b.span().len();
    let len = if a_len < b_len {
        a_len
    } else {
        b_len
    };

    loop {
        if i == len {
            break;
        }
        sum = sum + *a.span().at(i);
        i += 1;
    };
    let mut sum2 = 0_felt252;
    let mut i: u32 = 0;
    loop {
        if i == len {
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
            complex(array![1, 2, 4, 8, 16], array![1, 2, 3, 4, 5]) == (31, 15, 5, 5), 'it works!'
        );
    }
}
