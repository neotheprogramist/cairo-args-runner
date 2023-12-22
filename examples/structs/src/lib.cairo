#[derive(Drop)]
struct Nested {
    a: felt252,
    b: felt252,
}

#[derive(Drop)]
struct InputData {
    a: felt252,
    b: felt252,
    c: felt252,
    d: Nested,
}

fn main(x: InputData) -> felt252 {
    f(x)
}

fn f(x: InputData) -> felt252 {
    x.a + x.b + x.c + x.d.a + x.d.b
}

#[cfg(test)]
mod tests {
    use super::{f, InputData, Nested};

    #[test]
    fn it_works() {
        assert(f(InputData { a: 1, b: 2, c: 10, d: Nested { a: 5, b: 9 } }) == 27, 'it works!');
    }
}
