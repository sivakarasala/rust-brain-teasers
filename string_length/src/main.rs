const HELLO_WORLD : &str = "Halló heimur";

fn main() {
    println!("{} is {} characters long.", HELLO_WORLD, HELLO_WORLD.len());
    println!("{} is {} characters long.", HELLO_WORLD, HELLO_WORLD.chars().count());
}
