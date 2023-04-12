mod my_tests;
mod tests;

pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                int_to_char(
                    (((char_to_int(c) + key as i32) % 26) + 26) % 26,
                    c.is_lowercase(),
                )
            } else {
                c
            }
        })
        .collect()
}

fn char_to_int(char: char) -> i32 {
    char as i32 - {
        if char.is_lowercase() {
            'a'
        } else {
            'A'
        }
    } as i32
}

fn int_to_char(input: i32, is_lowercase: bool) -> char {
    (input + {
        if is_lowercase {
            'a'
        } else {
            'A'
        }
    } as i32) as u8 as char
}
