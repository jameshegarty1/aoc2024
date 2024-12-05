use std::collections::HashMap;
use atoi::atoi;

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

                let mut void = false;
                for i in 0..numbers.len() {
                    let page_num = numbers[i];
                    let prev = &numbers[..i];
                    match rules.get(&page_num) {
                        Some(vector) => {
                            vector
                                .iter()
                                .for_each(|num| {
                                    prev
                                        .iter()
                                        .for_each(|prev_num| {
                                            if prev_num == num {
                                                print!("Number is {} but it needs to go before {} as per rule.\n", num, prev_num);
                                                print!("Update {:?} is void.\n", numbers);
                                                void = true;
                                            }
                                        })
                                    
                                })
                        },
                        None => println!("Page has no rules.")
                    }
                }

                if !void {
                    let midpoint = numbers[numbers.len()/2];
                    solution += midpoint;
                }
            }
        }
    print!("{}\n", solution);
}
