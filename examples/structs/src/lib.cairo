use core::array::ArrayTrait;
#[derive(Drop, Serde)]
struct Nested {
    a: felt252,
    b: felt252,
    c: Array<felt252>,
}

#[derive(Drop, Serde)]
struct InputData {
    a: felt252,
    b: felt252,
    c: felt252,
    d: Nested,
}

fn main(x: Array<felt252>) -> felt252 {
    let mut x_span = x.span();
    let deserialized_struct: InputData = Serde::<InputData>::deserialize(ref x_span).unwrap();
    f(deserialized_struct)
}

fn f(x: InputData) -> felt252 {
    let mut partial_result = x.a + x.b + x.c + x.d.a + x.d.b;
    let mut i = 0;
    loop {
        if i == x.d.c.len() {
            break;
        }
        partial_result += *x.d.c[i];
        i += 1;
    };
    partial_result
}

#[cfg(test)]
mod tests {
    use super::{f, InputData, Nested};

    #[test]
    fn it_works() {
        assert(
            f(InputData { a: 1, b: 2, c: 10, d: Nested { a: 5, b: 9, c: array![1, 2, 3] } }) == 33,
            'it works!'
        );
    }
}
