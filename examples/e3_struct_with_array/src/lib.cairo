struct InputData {
    x: Array<felt252>,
}

#[derive(Drop, PartialEq)]
struct OutputData {
    x: felt252,
}

fn main(a: InputData) -> OutputData {
    f(a)
}

fn f(a: InputData) -> OutputData {
    let x_span = a.x.span();
    let mut sum_x = 0;
    let mut i = 0;
    loop {
        if i >= x_span.len() {
            break;
        }
        sum_x += *x_span[i];
        i += 1;
    };

    OutputData { x: sum_x }
}

#[cfg(test)]
mod tests {
    use super::{f, InputData, OutputData};

    #[test]
    fn it_works() {
        assert(f(InputData { x: array![1, 2, 3] }) == OutputData { x: 6 }, 'it works!');
    }
}
