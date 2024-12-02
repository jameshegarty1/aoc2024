pub fn main() {
    let (mut a, mut b) = (Vec::new(), Vec::new());
    //Length of number in bytes from each input
    let num_len = include_bytes!("../input")
        .iter()
        .position(|&b| b == b' ')
        .unwrap();

    print!("{}", num_len);

    //iterate over the bytes, insert them into the vectors 
    for line in include_bytes!("../input")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty()) {
            let a_num: usize = atoi::atoi::<usize>(&line[0..num_len]).unwrap(); 
            let b_num: usize = atoi::atoi::<usize>(&line[num_len+3..]).unwrap(); 

            a.push(a_num);
            b.push(b_num);
    }

    a.sort_unstable();
    b.sort_unstable();


    for (x, y) in a.iter().zip(b.iter()) {
        println!("{} {}", x, y);
        println!("Diff: {}", x.abs_diff(*y));
    }

    println!(
        "{}",
        a.iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum::<usize>());


//    print!("{}", String::from_utf8_lossy(bytes));
}
