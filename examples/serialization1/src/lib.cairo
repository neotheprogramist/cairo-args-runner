use core::array::ArrayTrait;
fn main(a: felt252, b: felt252, c: felt252, d: Array<felt252>) -> (felt252, felt252, felt252, felt252) {
    f(a, b, c, d)
}

fn f(a: felt252, b: felt252, c: felt252, d: Array<felt252>) -> (felt252, felt252, felt252, felt252) {
    (a, b, c, d.len().into())
}

#[cfg(test)]
mod tests {
    use super::f;

    #[test]
    fn it_works() {
        assert(f(1, 2, 3, array![4]) == (1, 2, 3, 1), 'it works!');
    }
}
