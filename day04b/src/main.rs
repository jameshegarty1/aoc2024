use std::collections::HashMap;

#[derive(Debug)]
enum XMASPattern {
    TopM,
    TopS,    
    LeftM,
    LeftS,   
}

impl XMASPattern {
    fn get_pattern(&self) -> [(i32, i32); 4] {
        [(1, 1), (1, -1), (-1, 1), (-1, -1)]
    }

    fn get_targets(&self) -> [char; 4] {
        match self {
            XMASPattern::TopM => ['S', 'S', 'M', 'M'],  
            XMASPattern::TopS => ['M', 'M', 'S', 'S'],  
            XMASPattern::LeftM => ['S', 'M', 'S', 'M'], 
            XMASPattern::LeftS => ['M', 'S', 'M', 'S'], 
        }
    }
}

const PATTERNS: [XMASPattern; 4] = [
    XMASPattern::TopM,
    XMASPattern::TopS,
    XMASPattern::LeftM,
    XMASPattern::LeftS,
];


fn check_xmas(wordsearch: &HashMap<(i32,i32), char>, start_pos: (i32, i32), pattern: &XMASPattern) -> bool {
    let directions = pattern.get_pattern();
    let targets = pattern.get_targets();
    
    for (i,dir) in directions.iter().enumerate() {
        match wordsearch.get(&(start_pos.0+dir.0, start_pos.1+dir.1)) {
            Some(value) => {
                if *value == targets[i] {
                    continue;
                } else {
                    return false;
                }
            },
            None => return false,
        }
    }

    true

    
}


fn main() {
    let mut wordsearch = HashMap::new();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    include_bytes!("../input")
        .iter()
        .for_each(|b|{
            if *b == b'\n' {
                y += 1;
                x = 0;
            } else {
                wordsearch.insert((x.clone(),y.clone()), *b as char );
                x +=1;
            }
        });

    let mut count = 0;
    for (&pos, &c) in wordsearch.iter() {
        if c == 'A' {
            for pattern in PATTERNS {
                if check_xmas(&wordsearch, pos, &pattern) {
                    count += 1;
                }
            }
        }
    }

    print!("{}\n", count);
}
