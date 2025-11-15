use std::num::Wrapping;

fn main() {
    // let mut counter: i8 = 0;
    // loop {
    //     println!("{}", counter);
    //     counter += 1;
    // }
    let mut counter = Wrapping(0i8);
    loop {
        println!("{}", counter);
        counter += Wrapping(1i8);
    }

    // if let Some(n) = x.checked_add(b) {
    
    // } else {

    // }
}
