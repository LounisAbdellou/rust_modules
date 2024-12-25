use ftkit::ARGS;
use unicode_width::UnicodeWidthStr;

fn display_paragraphs(columns: usize, vector: Vec<String>) {
    let mut j = 0;
    let mut line_length = 0;

    for mut i in 0..vector.len() {
        let word_len = UnicodeWidthStr::width(vector[i].as_str());
        let mut word_count = i - j;

        if line_length + word_len + word_count >= columns || vector[i] == "\n" {
            let mut whitespaces = 0;
            if i == vector.len() - 1 {
                whitespaces = word_count - 1;
            } else if columns >= line_length {
                whitespaces = columns - line_length
            }

            if word_count > 1 {
                word_count -= 1;
            }

            while j < i {
                let mut k = 0;

                print!("{}", vector[j]);
                j += 1;

                if i != j {
                    while k < whitespaces / word_count {
                        print!(" ");
                        k += 1;
                    }

                    whitespaces -= whitespaces / word_count;
                    word_count -= 1;
                }
            }
            print!("\n");

            if vector[i] == "\n" && i < vector.len() - 1 {
                print!("\n");
                i += 1;
                j += 1;
            }

            line_length = UnicodeWidthStr::width(vector[i].as_str());
            continue;
        }
        line_length += word_len;
    }
}

fn main() {
    if ARGS.len() != 2 {
        panic!("Wrong number of argument.");
    }

    let mut is_prev_nl = false;
    let mut vector: Vec<String> = Vec::new();
    loop {
        let line = ftkit::read_line();
        if line.is_empty() {
            break;
        } else if line == "\n" && is_prev_nl {
            continue;
        } else if line == "\n" && !is_prev_nl {
            vector.push(line.to_string());

            is_prev_nl = true;
            continue;
        }

        is_prev_nl = false;
        let splitted_line = line.split_whitespace();
        for part in splitted_line {
            vector.push(part.to_string());
        }
    }
    vector.push("\n".to_string());

    display_paragraphs(ARGS[1].parse::<usize>().unwrap(), vector);
}
