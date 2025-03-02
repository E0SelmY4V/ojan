use ojan::*;

fn digitify(digit_char: char) -> u16 {
    match digit_char {
        'X' => 10,
        _ => digit_char as u16 - '0' as u16,
    }
}
fn charify(digit: u16) -> char {
    match digit {
        10 => 'X',
        _ => ('0' as u8 + digit as u8) as char,
    }
}

pub fn main() {
    let iner = input::new();
    let mut code: u16 = 0;
    let mut power: u16 = 0;
    let isbn = iner.read_line();
    let mut new_isbn = vec![];
    let mut f = false;
    for digit_char in isbn.chars().take(13) {
        new_isbn.push(digit_char);
        if digit_char == '-' {
            continue;
        }
        let digit = digitify(digit_char);
        if power == 9 {
            f = digit == code;
            break;
        }
        power += 1;
        code = (code + (digit * power)) % 11;
    }
    new_isbn.pop();
    let tip = if f {
        String::from("Right")
    } else {
        new_isbn.push(charify(code));
        String::from_iter(new_isbn)
    };
    print!("{tip}");
}
