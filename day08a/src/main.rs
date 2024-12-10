use std::collections::{HashMap,HashSet};
use gcd::euclid_u32;

#[derive(Hash,PartialEq,Eq,Copy,Clone,Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        (dx * dx + dy * dy).sqrt()
    }

}

#[derive(Copy,Clone,Debug)]
struct Antenna {
    location: Point,
    frequency: char,
}

#[derive(Debug)]
struct Map {
    max_x: i32,
    max_y: i32,
    antennas: HashMap<char, Vec<Antenna>>,
}

impl Map {
    fn from_bytes(bytes: &[u8]) -> Self {
        let max_y = (bytes.iter().filter(|b| **b == b'\n').count() - 1) as i32;
        let max_x = (bytes.iter().position(|b| *b == b'\n').unwrap() - 1) as i32;
        let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        bytes
            .iter()
            .for_each(|&b| {
                let character = b as char;

                if (character).is_alphanumeric() {
                    let antenna = Antenna { 
                        location: Point{x,y}, 
                        frequency: character
                    };
                    antennas
                        .entry(character)
                        .and_modify(|vec| vec.push(antenna))
                        .or_insert(vec![antenna]);

                }
                if character == '\n' {
                    y += 1;
                    x = 0;
                } else {
                    x += 1;

                }

        });

        Map { max_x, max_y, antennas }
    }

    fn find_antinodes (&self) -> HashSet<Point> {
        let mut antinodes_set: HashSet<Point> = HashSet::new();
        for (frequency,antennas) in &self.antennas {    
            for i in 0..antennas.len() {
                for j in i+1..antennas.len() {
                    let a1 = &antennas[i];
                    let b1 = &antennas[j];
                    println!("Checking antennas at ({},{}) and ({},{}) with frequency {}", 
    a1.location.x, a1.location.y, b1.location.x, b1.location.y, a1.frequency);

                    let antinodes = self.find_antinodes_of(&a1.location, &b1.location);
                    antinodes_set.extend(antinodes);
                }
            }
        } 
        antinodes_set
    }

    fn find_antinodes_of(&self, a1: &Point, b1: &Point) -> Vec<Point> { 
        let vec_pos = (a1.x - b1.x, a1.y - b1.y); 
        let gcd = euclid_u32(vec_pos.0.abs() as u32, vec_pos.1.abs() as u32) as i32;
        let norm_vec_pos = ((vec_pos.0 / gcd as i32) , (vec_pos.1 / gcd as i32));

        println!("Vector: {:?}, GCD: {}, Norm: {:?}",vec_pos, gcd, norm_vec_pos);

        //Antinode are found along the vector at points:
        //antenna_1 + 2 * gcd(antenna_1, antenna_2)
        //antenna_2 + 2 * gcd
        let antinode_1 = Point { 
            x: a1.x + (gcd/2)*norm_vec_pos.0,  // halfway to b1
            y: a1.y + (gcd/2)*norm_vec_pos.1
        };
        let antinode_2 = Point {
            x: a1.x + 2*gcd*norm_vec_pos.0,    // double distance from a1
            y: a1.y + 2*gcd*norm_vec_pos.1
        };
    
    // Second set of antinodes: from b1's perspective
        let antinode_3 = Point {
            x: b1.x - (gcd/2)*norm_vec_pos.0,  // halfway back to a1
            y: b1.y - (gcd/2)*norm_vec_pos.1
        };
        let antinode_4 = Point {
            x: b1.x - 2*gcd*norm_vec_pos.0,    // double distance back from b1
            y: b1.y - 2*gcd*norm_vec_pos.1
        };

        println!("Potential antinodes: {:?}, {:?}, {:?}, {:?}", 
    antinode_1, antinode_2, antinode_3, antinode_4);
       
        let mut antinodes_vec: Vec<Point> = Vec::with_capacity(4);
        for antinode in [antinode_1, antinode_2, antinode_3, antinode_4] {
            if self.is_inbounds(&antinode) {
                println!("Antinode found! {:?}", antinode);
                antinodes_vec.push(antinode);
            }
        }

        println!("Register antinodes: {:?}", antinodes_vec);
        antinodes_vec        
    }

    fn is_inbounds(&self, point: &Point) -> bool {
        point.x <= self.max_x && point.x >= 0 && point.y <= self.max_y && point.y >= 0
    }
 
}

fn solve() -> usize {
    let map = Map::from_bytes(include_bytes!("../example")); 
    println!("Map: {:?}", map);


    let nodes = map.find_antinodes();
    nodes.len()
}

fn main() {
    println!("{}", solve())
}
