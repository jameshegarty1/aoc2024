use std::collections::HashMap;
use atoi::atoi;

fn eval_rules(rules: &HashMap::<i32, Vec<i32>>, pages: &Vec<i32>) -> (bool, Option<(usize,usize)>) {
    let mut void = false;
    let mut bad_indices = None;
    for i in 0..pages.len() {
        let page_num = pages[i];
        let prev = &pages[..i];
        match rules.get(&page_num) {
            Some(vector) => {
                vector
                    .iter()
                    .for_each(|num| {
                        prev
                            .iter()
                            .enumerate()
                            .for_each(|(j,prev_num)| {
                                if prev_num == num {
                                    print!("Number {} needs to go before {}.\n", page_num, prev_num);
                                    print!("Update {:?} is void. Bad indices are {},{}\n", pages, j, i);
                                    void = true;
                                    bad_indices = Some((j,i));
                                }
                            })
                        
                    })
            },
            None => {}
        }
    }
    (void, bad_indices)
}

fn correct_pages(rules: &HashMap::<i32, Vec<i32>>, pages: &Vec<i32>) -> Vec<i32> {
    print!("Pages before: {:?}\n", pages);
    let mut void = true;
    let mut bad_indices: Option<(usize,usize)>;
    let mut new_pages: Vec<i32> = pages.clone();
    while void {
        (void, bad_indices) = eval_rules(rules, &new_pages); 
        match bad_indices {
            Some((a,b)) => {
                let before_element = new_pages[a];
                let after_element = new_pages[b];
                new_pages[a] = after_element;
                new_pages[b] = before_element;
            },
            None => print!("Pages are now in a correct order!\n")
        }
        print!("Pages after: {:?}\n", new_pages)
    }

    new_pages
}

fn main() {
    let mut parsing_rules = true;
    let mut rules: HashMap::<i32, Vec<i32>> = HashMap::new();
    let mut solution = 0;
    for line in include_bytes!("../input")
        .split(|&b| b == b'\n') {
            if line.is_empty() {
                parsing_rules = false;
                continue;
            }

            if parsing_rules {
                let parts: Vec<&[u8]> = line.split(|b| *b == b'|').collect();
                let num1: i32 = String::from_utf8_lossy(parts[0]).parse().unwrap();
                let num2: i32 = String::from_utf8_lossy(parts[1]).parse().unwrap();

                rules.entry(num1).and_modify(|vec| vec.push(num2)).or_insert(vec![num2]);

                //print!("RulesMap: {:?}\n", rules);

                //print!("{} {}\n", num1, num2);
            } else {
                let numbers: Vec<i32> = line
                    .split(|b| *b == b',')
                    .map(|slice| atoi::<i32>(slice).unwrap())
                    .collect();

                print!("\nEvaluating pages: {:?}\n", numbers);

                let (void,_) = eval_rules(&rules, &numbers);
                
                if void {
                    let corrected = correct_pages(&rules, &numbers);
                    let midpoint = corrected[corrected.len()/2];
                    solution += midpoint;
                } else {
                    print!("Pages are correct to begin with.\n");
                }
            }
        }
    print!("{}\n", solution);
}
