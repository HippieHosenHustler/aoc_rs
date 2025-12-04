pub fn lines_to_vec(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

/// Macro to automatically register day solutions
#[macro_export]
macro_rules! register_days {
    // Patern: match one or more "number => path" pairs
    ($day_var:ident, $input_var:ident, { $( $day:expr => $module:ident ),+ $(,)? }) => {
        match $day_var {
            // For each pair, generate a match arm
            $(
                $day => $module::solve($input_var),
            )+
            // Default case for unimplemented days
            _ => {
                eprintln!("Error: Day {} is not yet implemented.", $day_var);
                std::process::exit(1);
            }
        }
    };
}
