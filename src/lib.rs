static dictionary: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

static padding_char: char = '=';

pub fn base64_encode(input: &str) -> String {
    let mut result = String::new();
    let mut counter = 0;
    let mut left_carry: u8 = 0;

    if input.is_empty() {
        return result;
    }

    for char in input.chars() {
        counter += 1;

        if counter == 1 {
            let (char, carry) = handle_first_char_in_group(char);
            result.push(char);
            left_carry = carry;
        }

        if counter == 2 {
            let (char, carry) = handle_second_char_in_group(char, left_carry);
            result.push(char);
            left_carry = carry;
        }

        if counter == 3 {
            let (first_char, second_char) = handle_third_char_in_group(char, left_carry);

            result.push(first_char);
            result.push(second_char);

            counter = 0;
            left_carry = 0;
        }
    }

    //add padding
    if counter != 0 {
        let padding = calculate_padding(counter);
        if padding == 2 {
            let char = handle_padding_first_char(left_carry);
            result.push(char);
            result.push(padding_char);
            result.push(padding_char);
        } else {
            let char = handle_padding_second_char(left_carry);
            result.push(char);
            result.push(padding_char);
        }
    }

    println!("{}", result);
    result
}

fn handle_first_char_in_group(first_char: char) -> (char, u8) {
    let char = dictionary[(first_char as u8 >> 2) as usize];
    let carry = ((first_char as u8) & 0x3) as u8;

    (char, carry)
}

fn handle_second_char_in_group(second_char: char, carry: u8) -> (char, u8) {
    let char = dictionary[(((second_char as u8) >> 4) | (carry << 4)) as usize];
    let carry = (second_char as u8) & 0xF;
    (char, carry)
}

fn handle_third_char_in_group(third_char: char, carry: u8) -> (char, char) {
    let first_char = dictionary[((carry << 2) | ((third_char as u8) >> 6)) as usize];
    let second_char = dictionary[((third_char as u8) & 0x3F) as usize];
    (first_char, second_char)
}

fn handle_padding_first_char(carry: u8) -> char {
    let char = dictionary[(0x00 | (carry << 4)) as usize];
    char
}

fn handle_padding_second_char(carry: u8) -> char {
    let char = dictionary[((carry << 2) | (0x00)) as usize];
    char
}

fn calculate_padding(counter: u8) -> u8 {
    3 - counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(&base64_encode("crab"), "Y3JhYg==");
        assert_eq!(
            &base64_encode("the brown fox jump over the lazy dog!"),
            "dGhlIGJyb3duIGZveCBqdW1wIG92ZXIgdGhlIGxhenkgZG9nIQ=="
        );
        assert_eq!(&base64_encode(""), "");
        assert_eq!(&base64_encode("a"), "YQ==");
    }
}
