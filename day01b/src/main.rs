 use std::collections::HashMap;

pub fn main() {
    //Vectors A and B
    let (mut v_a, mut v_b) = (Vec::new(), Vec::new());

    //Hash map for occurrences of each element
    let (mut m_a, mut m_b) = (HashMap::<usize,usize>::new(), HashMap::<usize,usize>::new());

    //Length of number in bytes from each input
    let num_len = include_bytes!("../input")
        .iter()
        .position(|&b| b == b' ')
        .unwrap();

    //iterate over the bytes, insert them into the vectors 
    for line in include_bytes!("../input")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty()) {
            let a_num: usize = atoi::atoi::<usize>(&line[0..num_len]).unwrap(); 
            let b_num: usize = atoi::atoi::<usize>(&line[num_len+3..]).unwrap(); 

            v_a.push(a_num);
            v_b.push(b_num);
    }

    v_a.sort_unstable();
    v_b.sort_unstable();

    //Iterate over the sorted vectors. Add count to map
    for u_a in v_a.into_iter() {
        m_a.entry(u_a).and_modify(|e| *e += 1).or_insert(1);
    } 
    
    for u_b in v_b.into_iter() {
        m_b.entry(u_b).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut similarity = 0;

    //Iterate over keys in A, find similarity in B
    for key in m_a.keys() {
        match m_b.get(key) {
            Some(value) => {similarity += key * value},
            None => {}
        }
    }



    print!("{}\n", similarity);
}
