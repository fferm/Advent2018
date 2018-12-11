use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
struct Coord {
    x: isize,
    y: isize
}

fn main() {
    let grid_serial_number = 9435;

    let mut power_levels = HashMap::new();

    for x in 1..301 {
        for y in 1..301{
            let c = Coord{x: x, y: y};
            power_levels.insert(c, power_level(c, grid_serial_number));
        }
    }

    let mut max_power = std::isize::MIN;
    let mut max_top_left = Coord{x: 0, y: 0};

    for y in 1..299 {
        for x in 1..299 {
            let c = Coord{x: x, y: y};

            let total_power = grid_power(c, &power_levels);

            if total_power > max_power {
                max_power = total_power;
                max_top_left = c.clone();
            }
        }
    }

    for y in max_top_left.y..max_top_left.y+3 {
        for x in max_top_left.x..max_top_left.x+3 {
            let c = Coord{x: x, y: y};
            print!("{:3} ", &power_levels.get(&c).unwrap());
        }

        println!();
    }
    println!("Max power: {} at {:?}", max_power, max_top_left);


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

fn grid_power(c: Coord, power_levels: &HashMap<Coord, isize>) -> isize {
    let mut total_power = 0;
    for dx in 0..3 {
        for dy in 0..3 {
            total_power = total_power + power_levels.get(&Coord{x: c.x + dx, y: c.y + dy}).unwrap()
        }
    }
    return total_power;
}
