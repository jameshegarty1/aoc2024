use::std::collections::HashSet;

type Point = (usize, usize);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Guard {
    location: Point,
    orientation: Direction 
}

fn main() {
    let mut obstacle_map: HashSet<Point> = HashSet::new();
    let mut guard = Guard{
        location: (0,0),
        orientation: Direction::Up,
    };
    let mut x = 0;
    let mut y = 0;
    include_bytes!("../example")
        .iter()
        .for_each(|b| {
            if *b == b'#' {
                obstacle_map.insert((x,y));
            } else if *b == b'^' {
                guard.location = (x,y);
                guard.orientation = Direction::Up;
            } else if *b == b'>' {
                guard.location = (x,y);
                guard.orientation = Direction::Right;
            } else if *b == b'<' {
                guard.location = (x,y);
                guard.orientation = Direction::Left;
            } else if *b == b'v' {
                guard.location = (x,y);
                guard.orientation = Direction::Down;
            } 
            x += 1;

            if *b == b'\n' {
                y += 1;
                x = 0;
            }
            

        });

    println!("Guard: {:?}\n Obstacles: {:?}\n", guard, obstacle_map);

}
