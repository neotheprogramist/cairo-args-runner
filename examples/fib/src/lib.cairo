#[derive(Drop, Serde)]
struct InputData {
    n: felt252,
}

fn main(x: Array<felt252>) -> felt252 {
    let mut x_span = x.span();
    let deserialized_struct: InputData = Serde::deserialize(ref x_span).unwrap();
    fib(deserialized_struct.n)
}

fn fib(mut n: felt252) -> felt252 {
    let mut a: felt252 = 0;
    let mut b: felt252 = 1;
    loop {
        if n == 0 {
            break a;
        }
        n = n - 1;
        let temp = b;
        b = a + b;
        a = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::fib;

    #[test]
    fn it_works() {
        assert(fib(16) == 987, 'it works!');
    }
}
