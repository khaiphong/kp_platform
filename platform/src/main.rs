fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one in platform fn of lib crate is {}!", lib::add_one(num));
}
