fn main() {
    // 'bob: label

    'bob: loop {
        loop {
            loop {
                break;
            }
        }
    }
    'bob: loop {
        loop {
            loop {
                break 'bob;
            }
        }
    }
    'bob: loop {
        loop {
            loop {
                continue 'bob;
            }
        }
    }

    //while

    while dizzy() {}

    //via
    loop {
        //do stuff //do while
        if !dizzy() {
            break;
        }
        //do stuff
    }

    //for

    for num in [2, 4, 6].iter() {
        // do stuff with num
        //iter will iterate throw all values in list/array
    }

    let array = [(1, 2), (3, 4)];

    for (x, y) in array.iter() {
        // do stuff with num
    }

    for num in 0..50 {
        //range in python 0 to 49
    }

    for num in 0..=50 {
        //range in python 0 to 50
    }
}
