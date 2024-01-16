struct InputData {
    x: Array<felt252>,
    y: Array<felt252>,
}

#[derive(Drop, PartialEq)]
struct OutputData {
    x: felt252,
    y: felt252,
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

    let y_span = a.y.span();
    let mut sum_y = 0;
    let mut i = 0;
    loop {
        if i >= y_span.len() {
            break;
        }
        sum_y += *y_span[i];
        i += 1;
    };

    OutputData { x: sum_x, y: sum_y }
}

#[cfg(test)]
mod tests {
    use super::{f, InputData, OutputData};

    #[test]
    fn it_works() {
        assert(
            f(InputData { x: array![1, 2, 3], y: array![1, 3, 9] }) == OutputData { x: 6, y: 13 },
            'it works!'
        );
    }
}
