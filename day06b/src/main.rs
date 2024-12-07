use::std::collections::HashSet;

type Point = (i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Guard {
    position: Point,
    orientation: Direction 
}


fn is_out_of_bounds(max_x: &i32, max_y: &i32, position: &(i32,i32)) -> bool {
    position.0 >= 0 && position.0 <= *max_x && position.1 >=0 && position.1 <= *max_y
}


fn does_cause_loop(obstacle_map: &HashSet<Point>, position: &Point, guard: &Guard, max_x: &i32, max_y: &i32) -> bool {
    let mut new_obstacles: HashSet<Point> = obstacle_map.clone();
    let mut new_guard: Guard = guard.clone();
    new_obstacles.insert(*position);

    let mut guard_configurations: HashSet<Guard> = HashSet::new();
    let mut inbounds = true;

    while inbounds {
        if !guard_configurations.contains(&new_guard) {
            guard_configurations.insert(new_guard);
        } else {
            return true;
        }

        let motion = new_guard.orientation.get_translation();
        let next_x: i32 = i32::from(new_guard.position.0 + motion.0);
        let next_y: i32 = i32::from(new_guard.position.1 + motion.1);


        match new_obstacles.get(&(next_x, next_y)) {
            Some(_) => new_guard.orientation = new_guard.orientation.get_next_direction(),
            None => new_guard.position = (next_x, next_y),
        }
                
        inbounds = is_out_of_bounds(&max_x, &max_y, &new_guard.position);
        
    }
    false
}

fn main() {
    let mut obstacle_map: HashSet<Point> = HashSet::new();
    let mut empty_map: HashSet<Point> = HashSet::new();
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
            } else if *b == b'.' {
                empty_map.insert((x,y));
            }
            x += 1;

            if *b == b'\n' {
                y += 1;
                x = 0;
            }
            
            

        });

    let mut solution = 0;

    empty_map
        .iter()
        .for_each(|point| {
            if does_cause_loop(&obstacle_map, point, &guard, &max_x, &max_y) {
                solution += 1;
            }
        });

    println!("{}",solution);
}
