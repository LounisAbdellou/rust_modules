use std::cmp::Ordering;

fn main() {
    let to_guess = ftkit::random_number(0..100);

    println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");

    loop {
        let guess = ftkit::read_number();

        if guess.cmp(&to_guess) == Ordering::Equal {
            println!("That is right! The secret was indeed the number {}, which you have brilliantly discovered!", to_guess);
            break;
        } else if guess.cmp(&to_guess) == Ordering::Greater {
            println!("Sometimes I wonder whether I should retire. I would have guessed higher.");
        } else if guess.cmp(&to_guess) == Ordering::Less {
            println!("This student might not be as smart as I was told. This answer is obviously too weak.");
        }
    }
}
