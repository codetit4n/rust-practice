use std::io;

fn main() {
    println!("Convert to pig latin!");
    println!("Enter the string:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    input = input.trim().to_string();

    let s: Vec<&str> = input.split(' ').collect();

    let mut output = String::new();

    for word in s {
        if let Some(val) = pig_latinize_word(word) {
            output = format!("{} {}", output, val);
        }
    }

    output = output.trim().to_string();
    println!("{:?}", output);
}

fn pig_latinize_word(s: &str) -> Option<String> {
    let mut first_char = char::default(); //'\0'
    for c in s.chars() {
        first_char = c;
        break;
    }
    if first_char == char::default() {
        return None;
    }

    let mut to_return = s.to_string();
    if is_vovel(first_char) {
        to_return.push_str("-hay");
    }

    if is_consonant(first_char) {
        to_return = to_return[1..].to_string();
        to_return.push('-');
        to_return.push(first_char);
        to_return.push_str("ay");
    }

    Some(to_return)
}

fn is_vovel(c: char) -> bool {
    c == 'a'
        || c == 'e'
        || c == 'i'
        || c == 'o'
        || c == 'u'
        || c == 'A'
        || c == 'E'
        || c == 'I'
        || c == 'O'
        || c == 'U'
}

// any char that is not a vovel in a-z or A-Z
fn is_consonant(c: char) -> bool {
    c == 'b'
        || c == 'c'
        || c == 'd'
        || c == 'f'
        || c == 'g'
        || c == 'h'
        || c == 'j'
        || c == 'k'
        || c == 'l'
        || c == 'm'
        || c == 'n'
        || c == 'p'
        || c == 'q'
        || c == 't'
        || c == 'r'
        || c == 's'
        || c == 'v'
        || c == 'w'
        || c == 'x'
        || c == 'y'
        || c == 'z'
        || c == 'B'
        || c == 'C'
        || c == 'D'
        || c == 'F'
        || c == 'G'
        || c == 'H'
        || c == 'J'
        || c == 'K'
        || c == 'L'
        || c == 'M'
        || c == 'N'
        || c == 'P'
        || c == 'Q'
        || c == 'T'
        || c == 'R'
        || c == 'S'
        || c == 'V'
        || c == 'W'
        || c == 'X'
        || c == 'Z'
        || c == 'Y'
}
