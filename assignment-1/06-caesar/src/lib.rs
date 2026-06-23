pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn caesar(input: &str, shift: i32) -> String {
    let _ = (input, shift);
    // normalize shift to range [0, 25] using modulo
    // adding ALPHABET.len() before modulo handles negative shifts correctly
    let shift = ((shift % ALPHABET.len() as i32) + ALPHABET.len() as i32) % ALPHABET.len() as i32;
    
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                // find position in alphabet, apply shift, wrap around
                let pos = (c as usize - 'a' as usize) as i32;
                let new_pos = (pos + shift) % ALPHABET.len() as i32;
                ALPHABET.chars().nth(new_pos as usize).unwrap()
            } else if c.is_ascii_uppercase() {
                // same logic for uppercase, but convert back to uppercase
                let pos = (c as usize - 'A' as usize) as i32;
                let new_pos = (pos + shift) % ALPHABET.len() as i32;
                ALPHABET
                    .chars()
                    .nth(new_pos as usize)
                    .unwrap()
                    .to_ascii_uppercase()
            } else {
                // no change for non-letters
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_shift_three() {
        assert_eq!(caesar("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn shift_minus_one() {
        assert_eq!(caesar("abc", -1), "zab");
    }

    #[test]
    fn shift_twenty_seven_wraps_to_one() {
        assert_eq!(caesar("xyz", 27), "yza");
    }

    #[test]
    fn empty_input() {
        assert_eq!(caesar("", 5), "");
    }

    #[test]
    fn shift_zero_is_identity() {
        assert_eq!(caesar("Rust!", 0), "Rust!");
    }

    #[test]
    fn shift_twenty_six_is_identity() {
        assert_eq!(caesar("abc", 26), "abc");
    }

    #[test]
    fn non_letters_preserved() {
        assert_eq!(caesar("1 2 3 !", 5), "1 2 3 !");
    }

    #[test]
    fn large_negative_shift_wraps() {
        assert_eq!(caesar("abc", -27), "zab");
    }
}
