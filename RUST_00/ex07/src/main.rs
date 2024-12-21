use ftkit::ARGS;

fn main() {
    if ARGS.len() != 3 {
        return;
    }

    if ex07::strpcmp(ARGS[1].as_bytes(), ARGS[2].as_bytes()) {
        println!("yes");
    } else {
        println!("no");
    }
}
