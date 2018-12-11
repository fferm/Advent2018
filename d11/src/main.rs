use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
struct Coord {
    x: isize,
    y: isize
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
struct Grid {
    top_left: Coord,
    bottom_right: Coord,
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
                let top_left = Coord{x: x, y: y};
                let bottom_right = Coord{x: x + size - 1, y: y + size - 1};
                let grid = Grid{top_left: top_left, bottom_right: bottom_right};

                let total_power = grid_power(grid, &power_levels, &mut grids);

                if total_power > max_power {
                    max_power = total_power;
                    max_top_left = grid.top_left.clone();
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

fn grid_power(grid: Grid, power_levels: &HashMap<Coord, isize>, grids: &mut HashMap<Grid, isize>) -> isize {
    if grids.contains_key(&grid) {
        return *grids.get(&grid).unwrap();
    }



    let x_size = grid.bottom_right.x - grid.top_left.x + 1;
    let y_size = grid.bottom_right.y - grid.top_left.y + 1;

    let mut total_power = 0;

    if x_size == 1 && y_size == 1 {
        total_power = total_power + power_levels.get(&grid.top_left).unwrap();
    } else if x_size == 1 {
        let x = grid.top_left.x;
        let mid_y = grid.top_left.y + (y_size / 2);

        let grid1 = Grid{top_left: grid.top_left, bottom_right: Coord{x, y: mid_y - 1}};
        let grid2 = Grid{top_left: Coord{x: x, y: mid_y}, bottom_right: grid.bottom_right};

        total_power = total_power + grid_power(grid1, power_levels, grids);
        total_power = total_power + grid_power(grid2, power_levels, grids);

    } else if y_size == 1 {
        let mid_x = grid.top_left.x + (x_size / 2);
        let y = grid.top_left.y;

        let grid1 = Grid{top_left: grid.top_left, bottom_right: Coord{x: mid_x - 1, y: y}};
        let grid2 = Grid{top_left: Coord{x: mid_x, y: y}, bottom_right: grid.bottom_right};

        total_power = total_power + grid_power(grid1, power_levels, grids);
        total_power = total_power + grid_power(grid2, power_levels, grids);
    } else {
        let mid_x = grid.top_left.x + (x_size / 2);
        let mid_y = grid.top_left.y + (y_size / 2);

        let grid1 = Grid{top_left: grid.top_left, bottom_right: Coord{x: mid_x - 1, y: mid_y - 1}};
        let grid2 = Grid{top_left: Coord{x: mid_x, y: grid.top_left.y}, bottom_right: Coord{x: grid.bottom_right.x, y: mid_y - 1}};
        let grid3 = Grid{top_left: Coord{x: grid.top_left.x, y: mid_y}, bottom_right: Coord{x: mid_x - 1,y: grid.bottom_right.y}};
        let grid4 = Grid{top_left: Coord{x: mid_x, y: mid_y}, bottom_right: grid.bottom_right};

        total_power = total_power + grid_power(grid1, power_levels, grids);
        total_power = total_power + grid_power(grid2, power_levels, grids);
        total_power = total_power + grid_power(grid3, power_levels, grids);
        total_power = total_power + grid_power(grid4, power_levels, grids);

    }
    grids.insert(grid, total_power);

    return total_power;
}








