use::std::collections::HashSet;

type Point = (i32, i32);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction{
    fn get_translation(&self) -> (i32,i32) {
        match self {
            Direction::Up => (0,-1),
            Direction::Down => (0,1),
            Direction::Left => (-1,0),
            Direction::Right => (1,0)
        } 
    }
    fn get_next_direction(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        } 
    }
}

#[derive(Debug)]
struct Guard {
    position: Point,
    orientation: Direction 
}


fn is_out_of_bounds(max_x: &i32, max_y: &i32, position: &(i32,i32)) -> bool {
    position.0 >= 0 && position.0 <= *max_x && position.1 >=0 && position.1 <= *max_y
}

fn main() {
    let mut obstacle_map: HashSet<Point> = HashSet::new();
    let mut guard = Guard{
        position: (0,0),
        orientation: Direction::Up,
    };

    let max_y: i32 = (include_bytes!("../input")
        .iter()
        .filter(|b| **b == b'\n')
        .count() - 1)
        .try_into().unwrap();

    let max_x: i32 = (include_bytes!("../input")
        .iter()
        .position(|b| *b == b'\n')
        .unwrap() - 1)
        .try_into().unwrap();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    include_bytes!("../input")
        .iter()
        .for_each(|b| {
            if *b == b'#' {
                obstacle_map.insert((x,y));
            } else if *b == b'^' {
                guard.position = (x,y);
                guard.orientation = Direction::Up;
            } else if *b == b'>' {
                guard.position = (x,y);
                guard.orientation = Direction::Right;
            } else if *b == b'<' {
                guard.position = (x,y);
                guard.orientation = Direction::Left;
            } else if *b == b'v' {
                guard.position = (x,y);
                guard.orientation = Direction::Down;
            } 
            x += 1;

            if *b == b'\n' {
                y += 1;
                x = 0;
            }
            
            

        });
    println!("max x {} max y {}",max_x,max_y);

    println!("Guard: {:?}\n Obstacles: {:?}\n", guard, obstacle_map);


    let mut guard_positions: HashSet<(i32,i32)> = HashSet::new();

    let mut inbounds = true;

    while inbounds {
        println!("Guard: {:?}", guard);
        if !guard_positions.contains(&guard.position) {
            guard_positions.insert(guard.position);
        }

        let motion = guard.orientation.get_translation();
        let next_x: i32 = i32::from(guard.position.0 + motion.0);
        let next_y: i32 = i32::from(guard.position.1 + motion.1);


        match obstacle_map.get(&(next_x, next_y)) {
            Some(_) => guard.orientation = guard.orientation.get_next_direction(),
            None => guard.position = (next_x, next_y),
        }
                

        inbounds = is_out_of_bounds(&max_x, &max_y, &guard.position);
        
    }

    println!("Distinct guard positions:\n{:?}", guard_positions);
    println!("Count: {}", guard_positions.len());



}
