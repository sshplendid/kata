fn main() {
    ascend(9);
    descend(9);
    sandglass(9);
}

fn ascend(i: i32) {
    for n in 0..i {
        {
            for k in 0..n + 1 {
                print!("*");
            }
        }
        println!("");
    }
}

fn descend(i: i32) {
    for n in (0..i).rev() {
        {
            for k in 0..n + 1 {
                print!("*");
            }
        }
        println!("");
    }
}

fn sandglass(i: i32) {
    let half = i / 2;
    for n in 0..i {
        let mut k = half - n;
        k = abs(k);
        // spaces
        for a in 0..(half - k) {
            print!(" ");
        }
        let mut k = i - n * 2;
        if (k < 0) {
            k = abs(k - 2);
        }
        // stars
        for b in 0..k {
            print!("*");
        }
        print!("\n");
    }
}

fn abs(number: i32) -> i32 {
    let mut result = number;
    if (result < 0) {
        result = result * (-1);
    }
    result
}
