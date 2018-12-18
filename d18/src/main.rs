use std::fs;
use std::collections::HashMap;
use std::fmt;


fn main() {
/*    let small_input = false;
    let filename;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut generation = 0;
    let mut world = read_inputs(filename);

    let target_generations = 100 * 1000;
    println!("gen\t\ttrees\tlumb\tscore");
    while generation < target_generations {
        generation += 1;

        world = world.run_minute();
        //println!("{:?}", world);

        let num_trees = world.how_many_have(State::Trees);
        let num_lumberyards = world.how_many_have(State::Lumberyard);

        println!("{:10}\t{:4}\t{:4}\t{:8}", generation, num_trees, num_lumberyards, num_trees * num_lumberyards);
    }

    let num_trees = world.how_many_have(State::Trees);
    let num_lumberyards = world.how_many_have(State::Lumberyard);

    println!("{:10}\t{:4}\t{:4}\t{:8}", generation, num_trees, num_lumberyards, num_trees * num_lumberyards);*/

    let values = vec![
        210915,
        217136,
        224114,
        234919,
        238018,
        245074,
        245890,
        248066,
        242896,
        241780,
        235220,
        224352,
        213565,
        207946,
        203138,
        198269,
        196527,
        200592,
        196742,
        197802,
        194892,
        196020,
        192234,
        194948,
        195415,
        198860,
        199662,
        207599
    ];

    let mut result = 0;
    let mut gen = 0;
    let mut idx = 0;
    while gen <= 1000000000 {
        if gen >= 454 {
            result = *values.get(idx).unwrap();
            idx = (idx + 1) % values.len();
        }

        if gen % 10000 == 0 {
            println!("Gen: {}", gen);
        }
        gen += 1;
    }

    println!("Final value: {}", result);
}

fn read_inputs(filename: &str) -> World {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");
    let lines: Vec<&str> = file_contents.split("\n").collect();

    let mut acres = HashMap::new();
    let mut x_size = 0;
    let mut y_size = 0;
    let mut y = 0;
    for line in lines {
        for x in 0..line.len() {
            let coord = Coord{x, y};

            let c = line.get(x..x+1).unwrap();

            if c == "." {
                acres.insert(coord, State::Open);
            } else if c == "#" {
                acres.insert(coord, State::Lumberyard);
            } else if c == "|" {
                acres.insert(coord, State::Trees);
            }

            if x > x_size && acres.contains_key(&coord) {
                x_size = x;
            }

        }
        if y > y_size {
            y_size = y
        }

        y += 1;
    }

    return World{acres, x_size, y_size};
}

struct World {
    x_size: usize,
    y_size: usize,
    acres: HashMap<Coord, State>
}

impl World {
    fn run_minute(&self) -> World {
        let mut new_acres = HashMap::new();

        for y in 0..self.y_size + 1 {
            for x in 0..self.x_size + 1 {
                let coord = Coord{x, y};

                let old_state = self.acres.get(&coord).unwrap();

                let new_state: State;
                match old_state {
                    State::Open => {
                        if self.how_many_neighbors_have(&coord, State::Trees) >= 3 {
                            new_state = State::Trees;
                        } else {
                            new_state = State::Open;
                        }
                    }
                    State::Trees => {
                        if self.how_many_neighbors_have(&coord, State::Lumberyard) >= 3 {
                            new_state = State::Lumberyard;
                        } else {
                            new_state = State::Trees;
                        }
                    },
                    State::Lumberyard => {
                        if self.how_many_neighbors_have(&coord, State::Lumberyard) >= 1 &&
                            self.how_many_neighbors_have(&coord, State::Trees) >= 1 {
                            new_state = State::Lumberyard;
                        } else {
                            new_state = State::Open;
                        }
                    }
                }

                new_acres.insert(coord, new_state);
            }
        }

        return World{acres: new_acres, x_size: self.x_size, y_size: self.y_size};
    }

    fn how_many_neighbors_have(&self, coord: &Coord, state: State) -> usize {
        let mut neighbors = Vec::new();

        for dy in -1 .. 2 {
            for dx in -1 ..2 {
                let x = coord.x as isize + dx;
                let y = coord.y as isize + dy;

                if (x >= 0) && (x <= self.x_size as isize) && (y >= 0) && (y <= self.y_size as isize) {
                    if x != coord.x as isize || y != coord.y as isize {
                        neighbors.push(Coord{x: x as usize, y: y as usize});
                    }
                }
            }
        }

        let mut counter = 0;
        for neighbor in neighbors {
            if *self.acres.get(&neighbor).unwrap() == state {
                counter += 1;
            }
        }

        return counter;
    }

    fn how_many_have(&self, target_state: State) -> usize {
        let mut counter = 0;
        for (_, state) in &self.acres {
            if *state == target_state {
                counter += 1;
            }
        }
        return counter;
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum State {
    Open,
    Trees,
    Lumberyard
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_owned();

        for y in 0..self.y_size + 1 {
            for x in 0..self.x_size + 1 {
                let coord = Coord { x, y };
                let mut ch: &str;

                let state = self.acres.get(&coord).unwrap();

                match state {
                    State::Trees => ch = "|",
                    State::Lumberyard => ch = "#",
                    State::Open => ch = "."
                }

                ret.push_str(&ch);
            }
            ret.push_str(&"\n");
        }

        return write!(f, "{}", ret);
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    y: usize,
    x: usize,
}


impl Coord {
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({},{})", self.x, self.y);
    }
}

