fn main() {
    const INPUT: &[u8] = include_bytes!("../input");

    let input: Vec<u8> = INPUT.iter()
        .filter(|&&b| b != b'\n' && b != b'\r')
        .cloned()
        .collect();

    let mut enabled = true;
    let mut solution = 0;
    let mut i = 0;
    while i < input.len() {
        let remaining = &input[i..];

        if remaining.starts_with(b"mul(") {
            i += 4;

            if !enabled {
                continue;
            };

            let next = &remaining[4..];
            //Need to check if next digit is is number.
            let byte_digits: Vec<u8> = next.iter().take_while(|b| b.is_ascii_digit()).cloned().collect();
            if byte_digits.len() > 0 && byte_digits.len() <= 3 {
                if next[byte_digits.len()] == b',' {
                    let next_byte_digits: Vec<u8> = next[byte_digits.len()+1..].iter().take_while(|b| b.is_ascii_digit()).cloned().collect();
                    if next_byte_digits.len() > 0 && next_byte_digits.len() <= 3 {

                        if next[byte_digits.len()+1+next_byte_digits.len()] == b')' {
                            let instruction_len = 2 + byte_digits.len() + next_byte_digits.len();

                            let num_1 = String::from_utf8(byte_digits).unwrap().parse::<u32>().unwrap();
                            let num_2 = String::from_utf8(next_byte_digits).unwrap().parse::<u32>().unwrap();

                            let result = num_1 * num_2;
                            solution += result;
                            i += instruction_len;

                        } 
                    }
                }
            } else {
                i += 4;
                
            }
        } else if remaining.starts_with(b"do()") {
            enabled = true;
            i += 4;
        } else if remaining.starts_with(b"don't()") {
            enabled = false;
            i += 6;
        } else {
            i += 1;
        }
    }

    print!("Soution = {}\n", solution);

}
