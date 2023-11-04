//|| {}

fn main() {
    let add = |x, y| x + y;

    let p = add(1, 2);
    println!("{}", p);

    let x = "rer".to_string();

    let f = || {
        println!("{}", x);
    };
    f();

    //by using move we can take ownership of any veriable that we inert as parameter

    let f = move || {
        println!("{}", x);
    };
    f();

    let mut v = vec![2, 4, 6];

    v.iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc, x| acc + x);
}
