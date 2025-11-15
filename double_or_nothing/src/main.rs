fn double_it<T>(n: T) -> T 
where T: std::ops::Mul<Output = T> + From<i32> {
    n * 2.into()
}

fn main() {
    println!("2 * 4 = {}", double_it(4));
}
