struct InputData {
    x: felt252,
    y: felt252,
    z: felt252,
}

#[derive(Drop, PartialEq)]
struct OutputData {
    x: felt252,
    y: felt252,
    z: felt252,
}

fn main(a: InputData) -> OutputData {
    f(a)
}

fn f(a: InputData) -> OutputData {
    OutputData { x: a.x, y: a.y, z: a.z }
}

#[cfg(test)]
mod tests {
    use super::{f, InputData, OutputData};

    #[test]
    fn it_works() {
        assert(f(InputData { x: 1, y: 2, z: 3 }) == OutputData { x: 1, y: 2, z: 3 }, 'it works!');
    }
}
