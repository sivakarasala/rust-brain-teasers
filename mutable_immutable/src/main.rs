fn main() {
    // let life_the_universe = &mut 41;
    // *life_the_universe += 1;
    let mut life = 40;
    let the_universe = &mut life;
    *the_universe += 2;
    println!("Life, the Universe and Everything: {}", the_universe);
}
