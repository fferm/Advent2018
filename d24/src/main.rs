extern crate regex;

use std::fs;
use std::fmt;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;


fn main() {
    let small_input = false;
    let filename;

    let immune_system: Army;
    let infection : Army;

    if small_input {
        immune_system = read_immune_small();
        infection = read_infection_small();
    } else {
        immune_system = read_immune_large();
        infection = read_infection_large();
    }

//    let nanobots = read_inputs(filename);
}

fn read_immune_small() -> Army {
//    17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
//    989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

    let mut g1 = Group::new(17, 5390, 4507, Attack::Fire, 2);
    g1.add_weakness(Attack::Radiation);
    g1.add_weakness(Attack::Bludgeoning);

    let mut g2 = Group::new(989, 1274, 25, Attack::Slashing, 3);
    g2.add_immune(Attack::Fire);
    g2.add_weakness(Attack::Bludgeoning);
    g2.add_weakness(Attack::Slashing);

    let mut immune = Army{groups: HashMap::new()};
    immune.groups.insert(1, g1);
    immune.groups.insert(2, g2);

    return immune;
}

fn read_immune_large() -> Army {
//    9936 units each with 1739 hit points (weak to slashing, fire) with an attack that does 1 slashing damage at initiative 11
//    2990 units each with 9609 hit points (weak to radiation; immune to fire, cold) with an attack that does 31 cold damage at initiative 1
//    2637 units each with 9485 hit points (immune to cold, slashing; weak to bludgeoning) with an attack that does 26 radiation damage at initiative 13
//    1793 units each with 2680 hit points (weak to bludgeoning; immune to cold) with an attack that does 13 bludgeoning damage at initiative 10
//    8222 units each with 6619 hit points (immune to fire, slashing) with an attack that does 6 bludgeoning damage at initiative 12
//    550 units each with 5068 hit points with an attack that does 87 radiation damage at initiative 19
//    950 units each with 8681 hit points (weak to radiation) with an attack that does 73 slashing damage at initiative 17
//    28 units each with 9835 hit points with an attack that does 2979 bludgeoning damage at initiative 3
//    3799 units each with 2933 hit points with an attack that does 7 slashing damage at initiative 16
//    35 units each with 8999 hit points (weak to bludgeoning; immune to radiation) with an attack that does 2505 cold damage at initiative 6

    let mut g1 = Group::new(9936, 1739, 1, Attack::Slashing, 11);
    g1.add_weakness(Attack::Slashing);
    g1.add_weakness(Attack::Fire);

    let mut g2 = Group::new(2990, 9609, 31, Attack::Cold, 1);
    g2.add_weakness(Attack::Radiation);
    g2.add_immune(Attack::Fire);
    g2.add_immune(Attack::Cold);

    // TODO: Fortsätt här

    let mut immune = Army{groups: HashMap::new()};
    immune.groups.insert(1, g1);
    immune.groups.insert(2, g2);

    return immune;
}

fn read_infection_small() -> Army {
//    801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
//    4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4

    let mut g1 = Group::new(801, 4706, 116, Attack::Bludgeoning, 1);
    g1.add_weakness(Attack::Radiation);

    let mut g2 = Group::new(4485, 2961, 12, Attack::Slashing, 4);
    g2.add_immune(Attack::Radiation);

    let mut infection = Army{groups: HashMap::new()};
    infection.groups.insert(1, g1);
    infection.groups.insert(2, g2);

    return infection;
}

struct Army {
    groups: HashMap<isize, Group>
}

struct Group {
    num_units: isize,
    hit_points_per_unit: isize,
    attack_damage_per_unit: isize,
    attack_type: Attack,
    initiative: isize,
    immunes: HashSet<Attack>,
    weaknesses: HashSet<Attack>
}

impl Group {
    fn new(num_units: isize, hit_points_per_unit: isize, attack_damage_per_unit: isize, attack_type: Attack, initiative: isize) -> Group {
        return Group{num_units, hit_points_per_unit, attack_damage_per_unit, initiative, immunes: HashSet::new(), weaknesses: HashSet::new(), attack_type}
    }
    fn add_weakness(&mut self, weakness: Attack) {
        self.weaknesses.insert(weakness);
    }
    fn add_immune(&mut self, immune: Attack) {
        self.immunes.insert(immune);
    }
}

enum Attack {
    Slashing,
    Fire,
    Radiation,
    Cold,
    Bludgeoning
}

