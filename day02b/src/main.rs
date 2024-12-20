use atoi::atoi;
pub fn main() {
    let mut n_safe = 0;

    //Loop over all input
    'outer: for line in include_bytes!("../input")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty()) {
            //Split bytes into vector of numbers
            let numbers: Vec<usize> = line
                .split(|&b| b == b' ')
                .map(|slice| atoi::<usize>(slice).unwrap())
                .collect();

            let is_safe = check_safety(&numbers);

            if is_safe {
                n_safe +=1;
                continue;
            } 

            //Check all removal possibilities
            for i in 0..numbers.len() {
                let mut temp_numbers = numbers.clone();
                temp_numbers.remove(i);
                if check_safety(&temp_numbers) {
                    n_safe += 1;
                    continue 'outer;
                }
            }
    }

    print!("{}\n", n_safe);

}

fn check_safety(report: &Vec<usize>) -> bool {
    let is_increasing: bool = report[1] > report[0];
        
    report.windows(2).all(|window| {
        let diff = window[0].abs_diff(window[1]);
        diff >= 1 && diff <= 3 && (window[1] > window[0]) == is_increasing
    })
}

