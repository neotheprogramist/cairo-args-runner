use core::array::ArrayTrait;
fn main(x: Array<felt252>) -> felt252 {
    sum(x)
}

fn sum(x: Array<felt252>) -> felt252 {
    let mut i: u32 = 0;
    let mut sum: felt252 = 0;
    loop {
        if i == x.len() {
            break sum;
        }
        sum = sum + *x.at(i);
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::sum;

    #[test]
    fn it_works() {
        assert(sum(array![1, 2, 3, 4, 5]) == 15, 'it works!');
    }
}