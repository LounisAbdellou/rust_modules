use std::fmt;

struct John;

impl fmt::Binary for John {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "Bip Boop?")
    }
}

impl fmt::Debug for John {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: fmt::Result;

        if f.alternate() {
            res = write!(
                f,
                "{}",
                "John, the man himself. He's handsome AND formidable."
            );
        } else {
            res = write!(f, "{}", "John, the man himself.");
        }

        res
    }
}

impl fmt::Display for John {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: fmt::Result;

        if f.precision() == Some(0) {
            res = write!(f, "{}", "Don't try to silence me!");
        } else {
            res = f.pad("Hey! I'm John.");
        }

        res
    }
}

fn main() {
    let john = John;

    println!("1. {john}");
    println!("2. |{john:<30}|");
    println!("3. |{john:>30}|");
    println!("4. {john:.6}");
    println!("5. {john:.0}");
    println!("6. {john:?}");
    println!("7. {john:#?}");
    println!("8. {john:b}");
}
