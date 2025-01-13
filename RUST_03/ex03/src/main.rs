use std::fmt::Debug;

trait FortyTwo {
    fn forty_two() -> Self;
}

impl FortyTwo for u32 {
    fn forty_two() -> Self {
        let value: Self = 42;

        return value;
    }
}

impl FortyTwo for String {
    fn forty_two() -> Self {
        let value: Self = String::from("FortyTwo");

        return value;
    }
}

fn print_forty_two<T: Debug + FortyTwo>() {
    let value = T::forty_two();

    println!("{value:?}");
}

fn main() {
    print_forty_two::<u32>();
    print_forty_two::<String>();
}
