fn main() {
    // let bunnies: i32 = 2;
    let mut bunnies = 32;
    println!("bunnies: {}", bunnies);
    let (bunnies1, carrots) = (8, 15);
    println!("bunnies1: {}, carrots: {}", bunnies1, carrots);

    // immutible due to which i got error

    let bunnies = 16;
    println!("bunnies: {}", bunnies);
}
