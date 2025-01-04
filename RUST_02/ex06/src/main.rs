use ftkit::ARGS;

#[derive(Debug)]
enum Token<'a> {
    Word(&'a str),
    RedirectStdout,
    RedirectStdin,
    Pipe,
}

fn next_token<'a>(s: &mut &'a str) -> Option<Token<'a>> {
    *s = s.trim_start();
    let mut it = s.char_indices();

    if s.is_empty() {
        return None;
    }

    if let Some(remaining) = s.strip_prefix("|") {
        *s = remaining;
        return Some(Token::Pipe);
    } else if let Some(remaining) = s.strip_prefix("<") {
        *s = remaining;
        return Some(Token::RedirectStdin);
    } else if let Some(remaining) = s.strip_prefix(">") {
        *s = remaining;
        return Some(Token::RedirectStdout);
    }

    let mut last_index = 0;
    while let Some((index, value)) = it.next() {
        last_index = index;

        if value.is_whitespace() || value == '|' || value == '<' || value == '>' {
            break;
        }
    }

    let (left, right) = s.split_at(if last_index < s.len() - 1 {
        last_index
    } else {
        s.len()
    });

    *s = right;

    return Some(Token::Word(left));
}

fn print_all_tokens(mut s: &str) {
    while let Some(token) = next_token(&mut s) {
        println!("{token:?}");
        // break;
    }
}

fn main() {
    if ARGS.len() != 2 {
        panic!("wrong number of arguments");
    }

    print_all_tokens(&ARGS[1]);
}
