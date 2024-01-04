#[derive(Drop, PartialEq)]
struct Response {
    a_len: felt252,
    sum_a: felt252,
    b_len: felt252,
    sum_b: felt252,
    c_len: felt252,
    sum_c: felt252,
}

fn main(a: Array<felt252>, b: Array<felt252>, c: Array<felt252>) -> Response {
    complex(a, b, c)
}

fn complex(a: Array<felt252>, b: Array<felt252>, c: Array<felt252>) -> Response {
    let a_len = a.span().len();
    let b_len = b.span().len();
    let c_len = c.span().len();

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

    let mut sum_c = 0_felt252;
    let mut i: u32 = 0;
    loop {
        if i == c_len {
            break;
        }
        sum_c = sum_c + *c.at(i);
        i += 1;
    };

    Response { a_len: a_len.into(), sum_a, b_len: b_len.into(), sum_b, c_len: c_len.into(), sum_c, }
}

#[cfg(test)]
mod tests {
    use super::{complex, Response};

    #[test]
    fn it_works() {
        assert(
            complex(
                array![1, 1, 1, 1, 1], array![2, 2, 2, 2, 2, 2], array![3, 3, 3]
            ) == Response { a_len: 5, sum_a: 5, b_len: 6, sum_b: 12, c_len: 3, sum_c: 9 },
            'it works!'
        );
    }
}
