use std::collections::HashMap;

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),   // right
    (0, -1),  // left
    (1, 0),   // down
    (-1, 0),  // up
    (1, 1),   // down-right
    (1, -1),  // down-left
    (-1, 1),  // up-right
    (-1, -1), // up-left
];

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

//Function to check if we have found an XMAS
//we take 1 step in the direction and check the map, see if we have M, then A then S

fn check_xmas(wordsearch: &HashMap<(i32,i32), char>, start_pos: (i32, i32), dir: (i32, i32) ) -> bool {
    let mut next_x = start_pos.0;
    let mut next_y = start_pos.1;

    print!("Found an {}\n", XMAS[0]);

    for i in 0..XMAS.len()-1 {
        next_x += dir.0;
        next_y += dir.1;
        match wordsearch.get(&(next_x, next_y)) {
            Some(value) => {
                if *value == XMAS[i+1] {
                    print!("Found an {}\n", XMAS[i+1]);
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
    for (&(x, y), &c) in wordsearch.iter() {
        if c == XMAS[0] {
            for dir in DIRECTIONS {
                if check_xmas(&wordsearch, (x,y), dir) {
                    print!("Found XMAS!\n");
                    count += 1;
                }
            }

        }
    }

    print!("{}\n", count);
}
