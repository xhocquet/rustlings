// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(25+25);
}
