// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let mut number: i32 = "3".parse().expect("Could not parse '3'");
    println!("Number {}", number);
    number = 3;
    println!("Number {}", number);
}
