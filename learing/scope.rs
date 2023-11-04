fn main() {
    let mut x = 5;
    {
        let y = 99;
        println!("{} {}", x, y);
    }
    // error scope y is limited
    // println!("{} {}", x, y);
    let x = 34;
    {
        let x: i32 = 99;
        println!("{} ", x);
    }
    println!("{}", x)
}
