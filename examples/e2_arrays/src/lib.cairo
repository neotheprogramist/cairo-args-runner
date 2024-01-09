#[derive(Drop, PartialEq)]
struct OutputData {
    x: felt252,
    y: felt252,
    z: felt252,
}

fn main(x: Array<felt252>, y: felt252, z: Array<felt252>) -> OutputData {
    f(x, y, z)
}

fn f(x: Array<felt252>, y: felt252, z: Array<felt252>) -> OutputData {
    let x_span = x.span();
    let mut sum_x = 0;
    let mut i = 0;
    loop {
        if i >= x_span.len() {
            break;
        }
        sum_x += *x_span[i];
        i += 1;
    };

    let z_span = z.span();
    let mut sum_z = 0;
    let mut i = 0;
    loop {
        if i >= z_span.len() {
            break;
        }
        sum_z += *z_span[i];
        i += 1;
    };
    OutputData { x: sum_x, y: y, z: sum_z }
}

#[cfg(test)]
mod tests {
    use super::{f, OutputData};

    #[test]
    fn it_works() {
        assert(f(array![1], 2, array![3]) == OutputData { x: 1, y: 2, z: 3 }, 'it works!');
    }
}
