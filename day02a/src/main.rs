use atoi::atoi;

pub fn main() {
    let mut n_safe = 0;
    //Length of number in bytes from each input
    for line in include_bytes!("../input")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty()) {
            print!("{}\n", String::from_utf8_lossy(line));

            let numbers: Vec<usize> = line
                .split(|&b| b == b' ')
                .map(|slice| atoi::<usize>(slice).unwrap())
                .collect();

            print!("Numbers: {:?}\n",numbers);

            let mut is_safe: bool = true;
            let is_increasing: bool = numbers[1] > numbers[0];
            print!("Initial Is increasing: {}\n", is_increasing);
            
            is_safe = numbers.windows(2)
                .all(|window| {
                print!("{} , {}\n", window[0], window[1]);
                let diff = window[0].abs_diff(window[1]);
                diff >= 1 && diff <= 3 && 
                (window[1] > window[0]) == is_increasing
            });

            print!("Is safe? {}\n\n", is_safe);

            if is_safe {
                n_safe += 1;
            } 
    }

    print!("Number of safe reports = {}\n", n_safe);

}
