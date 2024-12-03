fn main() {
    const INPUT: &[u8] = include_bytes!("../input");
    const EXAMPLE: &[u8] = include_bytes!("../example");

    let input: Vec<u8> = INPUT.iter()
        .filter(|&&b| b != b'\n' && b != b'\r')
        .cloned()
        .collect();

    let mut solution = 0;
    input
        .windows(4)
        .enumerate()
        .for_each(|(i,window)| {
            if window == b"mul(" {
                let remain = &input[i+4..];
                let is_instruction = true;

                //Need to check if next digit is is number.
                let byte_digits: Vec<u8> = remain.iter().take_while(|b| b.is_ascii_digit()).cloned().collect();
                if byte_digits.len() > 0 && byte_digits.len() <= 3 {
                    //Found a valud input
                    if remain[byte_digits.len()] == b',' {
                        let next_byte_digits: Vec<u8> = remain[byte_digits.len()+1..].iter().take_while(|b| b.is_ascii_digit()).cloned().collect();
                        if next_byte_digits.len() > 0 && next_byte_digits.len() <= 3 {
                            if remain[byte_digits.len()+1+next_byte_digits.len()] == b')' {
                                let num_1 = String::from_utf8(byte_digits).unwrap().parse::<u32>().unwrap();
                                let num_2 = String::from_utf8(next_byte_digits).unwrap().parse::<u32>().unwrap();

                                let result = num_1 * num_2;
                                solution += result;

                            } 
                        }
                    }
                }
            }
        });

    print!("Soution = {}\n", solution);

}
