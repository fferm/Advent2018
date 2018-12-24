use std::fmt;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cell::Cell;

fn main() {
    let small_input = true;

    let immune_system: Army;
    let infection : Army;

    if small_input {
        immune_system = read_immune_small();
        infection = read_infection_small();
    } else {
        immune_system = read_immune_large();
        infection = read_infection_large();
    }

    let mut cont = true;
    let mut round = 1;
    while cont {
        println!("Round {}", round);
        print_army("Immune System", &immune_system);
        print_army("Infection", &infection);

        // Target selection

        // Attack


        round += 1;
        cont = false;
    }
}

fn read_immune_small() -> Army {
//    17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
//    989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

    let mut g1 = Group::new(1, 17, 5390, 4507, Attack::Fire, 2);
    g1.add_weakness(Attack::Radiation);
    g1.add_weakness(Attack::Bludgeoning);

    let mut g2 = Group::new(2, 989, 1274, 25, Attack::Slashing, 3);
    g2.add_immune(Attack::Fire);
    g2.add_weakness(Attack::Bludgeoning);
    g2.add_weakness(Attack::Slashing);

    let mut immune = Army{groups: Vec::new()};
    immune.groups.push(g1);
    immune.groups.push(g2);

    return immune;
}

fn read_immune_large() -> Army {
//    9936 units each with 1739 hit points (weak to slashing, fire) with an attack that does 1 slashing damage at initiative 11
    let mut g1 = Group::new(1, 9936, 1739, 1, Attack::Slashing, 11);
    g1.add_weakness(Attack::Slashing);
    g1.add_weakness(Attack::Fire);

//    2990 units each with 9609 hit points (weak to radiation; immune to fire, cold) with an attack that does 31 cold damage at initiative 1
    let mut g2 = Group::new(1, 2990, 9609, 31, Attack::Cold, 1);
    g2.add_weakness(Attack::Radiation);
    g2.add_immune(Attack::Fire);
    g2.add_immune(Attack::Cold);

    //    2637 units each with 9485 hit points (immune to cold, slashing; weak to bludgeoning) with an attack that does 26 radiation damage at initiative 13
    let mut g3 = Group::new(3, 2637, 9485, 26, Attack::Radiation, 13);
    g3.add_immune(Attack::Cold);
    g3.add_immune(Attack::Slashing);
    g3.add_weakness(Attack::Bludgeoning);

//    1793 units each with 2680 hit points (weak to bludgeoning; immune to cold) with an attack that does 13 bludgeoning damage at initiative 10
    let mut g4 = Group::new(4, 1793, 2680, 13, Attack::Bludgeoning, 10);
    g4.add_weakness(Attack::Bludgeoning);
    g4.add_immune(Attack::Cold);

//    8222 units each with 6619 hit points (immune to fire, slashing) with an attack that does 6 bludgeoning damage at initiative 12
    let mut g5 = Group::new(5, 8222, 6619, 6, Attack::Bludgeoning, 12);
    g5.add_immune(Attack::Fire);
    g5.add_immune(Attack::Slashing);

//    550 units each with 5068 hit points with an attack that does 87 radiation damage at initiative 19
    let g6 = Group::new(6, 550, 5068, 87, Attack::Radiation, 19);

//    950 units each with 8681 hit points (weak to radiation) with an attack that does 73 slashing damage at initiative 17
    let mut g7 = Group::new(7,950, 8681, 73, Attack::Slashing, 17);
    g7.add_weakness(Attack::Radiation);

//    28 units each with 9835 hit points with an attack that does 2979 bludgeoning damage at initiative 3
    let g8 = Group::new(8, 28, 9835, 2979, Attack::Bludgeoning, 3);

//    3799 units each with 2933 hit points with an attack that does 7 slashing damage at initiative 16
    let g9 = Group::new(9, 3799, 2933, 7, Attack::Slashing, 16);

//    35 units each with 8999 hit points (weak to bludgeoning; immune to radiation) with an attack that does 2505 cold damage at initiative 6
    let mut g10 = Group::new(10, 35, 8999, 2505, Attack::Cold, 6);
    g10.add_weakness(Attack::Bludgeoning);
    g10.add_immune(Attack::Radiation);

    let mut immune = Army{groups: Vec::new()};
    immune.groups.push(g1);
    immune.groups.push(g2);
    immune.groups.push(g3);
    immune.groups.push(g4);
    immune.groups.push(g5);
    immune.groups.push(g6);
    immune.groups.push(g7);
    immune.groups.push(g8);
    immune.groups.push(g9);
    immune.groups.push(g10);

    return immune;
}

fn read_infection_small() -> Army {
//    801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
//    4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4

    let mut g1 = Group::new(1, 801, 4706, 116, Attack::Bludgeoning, 1);
    g1.add_weakness(Attack::Radiation);

    let mut g2 = Group::new(2, 4485, 2961, 12, Attack::Slashing, 4);
    g2.add_immune(Attack::Radiation);

    let mut infection = Army{groups: Vec::new()};
    infection.groups.push(g1);
    infection.groups.push(g2);

    return infection;
}

fn read_infection_large() -> Army {

//    1639 units each with 28720 hit points with an attack that does 27 cold damage at initiative 8
    let g1 = Group::new(1, 1639, 28720, 27, Attack::Cold, 8);

//    4968 units each with 16609 hit points (immune to slashing, bludgeoning, radiation) with an attack that does 6 fire damage at initiative 2
    let mut g2 = Group::new(2, 4968, 16609, 6, Attack::Fire, 2);
    g2.add_immune(Attack::Slashing);
    g2.add_immune(Attack::Bludgeoning);
    g2.add_immune(Attack::Radiation);

//    3148 units each with 48970 hit points (weak to fire, bludgeoning) with an attack that does 29 slashing damage at initiative 20
    let mut g3 = Group::new(3, 3148, 48970, 29, Attack::Slashing, 20);
    g3.add_weakness(Attack::Fire);
    g3.add_weakness(Attack::Bludgeoning);

//    1706 units each with 30069 hit points (immune to cold, bludgeoning) with an attack that does 29 fire damage at initiative 7
    let mut g4 = Group::new(4, 1706, 30069, 29, Attack::Fire, 7);
    g4.add_immune(Attack::Cold);
    g4.add_immune(Attack::Bludgeoning);

//    496 units each with 39909 hit points (immune to cold; weak to radiation) with an attack that does 133 bludgeoning damage at initiative 4
    let mut g5 = Group::new(5, 496, 39909, 133, Attack::Bludgeoning, 4);
    g5.add_immune(Attack::Cold);
    g5.add_weakness(Attack::Radiation);

//    358 units each with 17475 hit points with an attack that does 82 bludgeoning damage at initiative 5
    let g6 = Group::new(6, 358, 17475, 82, Attack::Bludgeoning, 5);

//    120 units each with 53629 hit points with an attack that does 807 fire damage at initiative 15
    let g7 = Group::new(7, 120, 53629, 807, Attack::Fire, 15);

//    402 units each with 44102 hit points (weak to slashing) with an attack that does 185 bludgeoning damage at initiative 14
    let mut g8 = Group::new(8, 402, 44102, 185, Attack::Bludgeoning, 14);
    g8.add_weakness(Attack::Slashing);

//    468 units each with 11284 hit points (weak to fire) with an attack that does 43 radiation damage at initiative 18
    let mut g9 = Group::new(9, 468, 11284, 43, Attack::Radiation, 18);
    g9.add_weakness(Attack::Fire);

//    4090 units each with 23075 hit points (immune to radiation) with an attack that does 10 bludgeoning damage at initiative 9
    let mut g10 = Group::new(10, 4090, 23075, 10, Attack::Bludgeoning, 9);
    g10.add_immune(Attack::Radiation);

    let mut infection = Army{groups: Vec::new()};
    infection.groups.push(g1);
    infection.groups.push(g2);
    infection.groups.push(g3);
    infection.groups.push(g4);
    infection.groups.push(g5);
    infection.groups.push(g6);
    infection.groups.push(g7);
    infection.groups.push(g8);
    infection.groups.push(g9);
    infection.groups.push(g10);

    return infection;
}

fn print_army(name: &str, army: &Army) {
    println!("{}", name);

    for group in &army.groups {
        println!("Group {} contains {} units", group.id, group.num_units.get());
    }
}

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Army {
    groups: Vec<Group>
}

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Group {
    id: isize,
    num_units: Cell<isize>,
    hit_points_per_unit: isize,
    attack_damage_per_unit: isize,
    attack_type: Attack,
    initiative: isize,
    immunes: Vec<Attack>,
    weaknesses: Vec<Attack>,
    alive: Cell<bool>,
    selected_enemy_idx: Cell<Option<usize>>
}

impl Group {
    fn new(id: isize, num_units: isize, hit_points_per_unit: isize, attack_damage_per_unit: isize, attack_type: Attack, initiative: isize) -> Group {
        return Group{id, num_units: Cell::new(num_units), hit_points_per_unit, attack_damage_per_unit, initiative, immunes: Vec::new(), weaknesses: Vec::new(), attack_type, alive: Cell::new(true), selected_enemy_idx: Cell::new(None)}
    }
    fn add_weakness(&mut self, weakness: Attack) {
        self.weaknesses.push(weakness);
    }
    fn add_immune(&mut self, immune: Attack) {
        self.immunes.push(immune);
    }

    fn get_effective_power(&self) -> isize {
        return self.num_units.get() * self.attack_damage_per_unit;
    }
}


#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
enum Attack {
    Slashing,
    Fire,
    Radiation,
    Cold,
    Bludgeoning
}

