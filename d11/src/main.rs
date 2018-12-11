use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
struct Coord {
    x: isize,
    y: isize
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
struct Grid {
    top_left: Coord,
    size: isize
}

fn main() {
    let grid_serial_number = 9435;

    let mut power_levels = HashMap::new();
    let mut grids = HashMap::new();

    for x in 1..301 {
        for y in 1..301{
            let c = Coord{x: x, y: y};
            power_levels.insert(c, power_level(c, grid_serial_number));
        }
    }

    let mut max_power = std::isize::MIN;
    let mut max_top_left = Coord{x: 0, y: 0};
    let mut max_size = 0;

    for size in 1..301 {
        println!("Analyzing size {}", size);
        for y in 1..302 - size {
            for x in 1..302 - size {
                let c = Coord{x: x, y: y};

                let total_power = grid_power(c, &power_levels, size, &mut grids);

                if total_power > max_power {
                    max_power = total_power;
                    max_top_left = c.clone();
                    max_size = size.clone();
                }
            }
        }

    }

    println!("Max power: {} at {:?} with size: {}", max_power, max_top_left, max_size);


}

fn power_level(c: Coord, grid_serial_number: isize) -> isize {
    let rack_id = c.x + 10;
    let mut power_level = rack_id * c.y;
    power_level = power_level + grid_serial_number;
    power_level = power_level * rack_id;

    power_level = power_level % 1000 / 100;

    power_level = power_level - 5;

    return power_level;
}

fn grid_power(c: Coord, power_levels: &HashMap<Coord, isize>, size: isize, grids: &mut HashMap<Grid, isize>) -> isize {
    let smaller_grid = Grid{top_left: c.clone(), size: size - 1};

    let mut total_power = 0;

    if grids.contains_key(&smaller_grid) {
        total_power = 0 + grids.get(&smaller_grid).unwrap();
        for dx in 0..size {
            total_power = total_power + power_levels.get(&Coord{x: c.x + dx, y: c.y + size - 1}).unwrap()
        }
        for dy in 0..size - 1 {
            total_power = total_power + power_levels.get(&Coord{x: c.x + size - 1, y: c.y + dy}).unwrap()
        }
    } else {
        for dx in 0..size {
            for dy in 0..size {
                total_power = total_power + power_levels.get(&Coord{x: c.x + dx, y: c.y + dy}).unwrap()
            }
        }
    }

    grids.insert(Grid{top_left:c.clone(), size: size}, total_power);

    return total_power;
}
