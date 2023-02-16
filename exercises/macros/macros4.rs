// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

macro_rules! mymax {
    ($v1:expr) => {
        $v1;
        $v1;
    };
}

fn main() {
    let mut a = 1;
    mymax!(a += 1);
    println!("{a}");

}
