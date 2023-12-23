#[macro_export]
macro_rules! felt_vec {
    ($($a:expr),*) => {
        vec![$(cairo_felt::Felt252::from($a)),*]
    };
}

#[macro_export]
macro_rules! arg_value {
    ($a:expr) => {
        cairo_lang_runner::Arg::Value(cairo_felt::Felt252::from($a))
    };
}

#[macro_export]
macro_rules! arg_value_vec {
    ($($a:expr),*) => {
        vec![$(cairo_lang_runner::Arg::Value(cairo_felt::Felt252::from($a))),*]
    };
}

#[macro_export]
macro_rules! arg_array {
    ($($a:expr),*) => {
        cairo_lang_runner::Arg::Array(felt_vec![$($a),*])
    };
}
