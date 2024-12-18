fn collatz(start: u32) {
    let mut n = start;

    loop {
        println!("{}", n);
        if n == 1 {
            break;
        }

        if n % 2 == 0 {
            n /= 2;
            continue;
        }

        n = (n * 3) + 1;
    }
}

fn main() {
    collatz(3);
}
