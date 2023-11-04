// Vactors act like  stack with the features like push ,pop

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(2);
    v.push(4);
    v.push(6);

    let x = v.pop(); //x is 6
}


fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(2);
    v.push(4);
    v.push(6);

    // let x: Option<i32> = v.pop(); //x is 6

    // println!("{}", x);
    println!("{}", v[1]);

    let mut c = vec![2, 4, 6];
    println!("{}", c[2]);
}
