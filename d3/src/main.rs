extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashMap;


#[derive(Debug)]
struct Claim {
    id: i32,
    top_left: Coord,
    size: Coord
}

#[derive(Debug, Hash, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coord {}

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let mut num_hits_per_coord = HashMap::new();

    for line in lines {

        println!("{}", line);

        let re = Regex::new("#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)").unwrap();
        let cap = re.captures_iter(line).next().unwrap();

        let claim = Claim {id: cap[1].parse().unwrap(), top_left: Coord{x: cap[2].parse().unwrap(), y: cap[3].parse().unwrap()}, size: Coord{x: cap[4].parse().unwrap(), y: cap[5].parse().unwrap()}};

        println!("{:?}", claim);
        for dx in 0..claim.size.x {
            let x = claim.top_left.x + dx;

            for dy in 0..claim.size.y {
                let y = claim.top_left.y + dy;

                let my_coord = Coord{x: x, y: y};

                let count = num_hits_per_coord.entry(my_coord).or_insert(0);
                *count += 1;

//                println!("{:?}    count: {}", &my_coord, count);
            }

        }
    }

    let mut num_coords_with_several_hits = 0;
    for (_coord, num_hits) in &num_hits_per_coord {
        if *num_hits > 1 {
            num_coords_with_several_hits = num_coords_with_several_hits + 1;
        }
    }

    println!("{} cords with several hits", num_coords_with_several_hits);



/*
    let c = Claim{ id: 231, top_left: Coord{x:1, y:2}, size: Coord{x: 2, y:3}};

    let s = Coord {x: 1, y: 2};

    println!("{:?}", c);
*/
}


