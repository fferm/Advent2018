use std::fmt;
use std::cell::Cell;

fn main() {
    let small_input = false;

    let mut sim: Sim;

    if small_input {
        sim = read_small();
    } else {
        sim = read_large();
    }

    let mut cont = true;
    let mut round = 1;
    while cont {
        println!("Round {}", round);
        println!("{:?}", &sim);
        println!();

        sim.select_targets();
        println!();

        sim.attack();
        println!();
        println!();

        round += 1;
        cont = sim.immune.has_alive() && sim.infection.has_alive();

        if round > 3500 {
            cont  = false;
        }
    }

    if sim.immune.has_alive() && !sim.infection.has_alive(){
        println!("Immune system wins with {} units alive", sim.immune.num_alive());
    } else if sim.infection.has_alive() && !sim.immune.has_alive() {
        println!("Infection wins with {} units alive", sim.infection.num_alive());
    } else {
        println!("Noone has won yet.  Immune: {}    Infection: {}", sim.immune.num_alive(), sim.infection.num_alive());
    }
    println!();
    println!();
}

fn read_small() -> Sim {
    let immune = read_immune_small();
    let infection = read_infection_small();

    return Sim{immune, infection};
}
fn read_immune_small() -> Army {
    let army = "Immune   ".to_owned();

//    17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
    let mut g1 = Group::new(army.clone(), 1, 17, 5390, 4507, Attack::Fire, 2);
    g1.add_weakness(Attack::Radiation);
    g1.add_weakness(Attack::Bludgeoning);

//    989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3
    let mut g2 = Group::new(army.clone(), 2, 989, 1274, 25, Attack::Slashing, 3);
    g2.add_immune(Attack::Fire);
    g2.add_weakness(Attack::Bludgeoning);
    g2.add_weakness(Attack::Slashing);

    let mut immune = Army{groups: Vec::new(), name: army.clone()};
    immune.groups.push(g1);
    immune.groups.push(g2);

    return immune;
}

fn read_infection_small() -> Army {
    let army = "Infection".to_owned();

//    801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
    let mut g1 = Group::new(army.clone(), 1, 801, 4706, 116, Attack::Bludgeoning, 1);
    g1.add_weakness(Attack::Radiation);

//    4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
    let mut g2 = Group::new(army.clone(), 2, 4485, 2961, 12, Attack::Slashing, 4);
    g2.add_immune(Attack::Radiation);
    g2.add_weakness(Attack::Fire);
    g2.add_weakness(Attack::Cold);

    let mut infection = Army{groups: Vec::new(), name: army.clone()};
    infection.groups.push(g1);
    infection.groups.push(g2);

    return infection;
}

fn read_large() -> Sim {
    let immune = read_immune_large();
    let infection = read_infection_large();

    return Sim{immune, infection};
}

fn read_immune_large() -> Army {
    let army = "Immune   ".to_owned();

//    9936 units each with 1739 hit points (weak to slashing, fire) with an attack that does 1 slashing damage at initiative 11
    let mut g1 = Group::new(army.clone(), 1, 9936, 1739, 1, Attack::Slashing, 11);
    g1.add_weakness(Attack::Slashing);
    g1.add_weakness(Attack::Fire);

//    2990 units each with 9609 hit points (weak to radiation; immune to fire, cold) with an attack that does 31 cold damage at initiative 1
    let mut g2 = Group::new(army.clone(), 2, 2990, 9609, 31, Attack::Cold, 1);
    g2.add_weakness(Attack::Radiation);
    g2.add_immune(Attack::Fire);
    g2.add_immune(Attack::Cold);

    //    2637 units each with 9485 hit points (immune to cold, slashing; weak to bludgeoning) with an attack that does 26 radiation damage at initiative 13
    let mut g3 = Group::new(army.clone(), 3, 2637, 9485, 26, Attack::Radiation, 13);
    g3.add_immune(Attack::Cold);
    g3.add_immune(Attack::Slashing);
    g3.add_weakness(Attack::Bludgeoning);

//    1793 units each with 2680 hit points (weak to bludgeoning; immune to cold) with an attack that does 13 bludgeoning damage at initiative 10
    let mut g4 = Group::new(army.clone(), 4, 1793, 2680, 13, Attack::Bludgeoning, 10);
    g4.add_weakness(Attack::Bludgeoning);
    g4.add_immune(Attack::Cold);

//    8222 units each with 6619 hit points (immune to fire, slashing) with an attack that does 6 bludgeoning damage at initiative 12
    let mut g5 = Group::new(army.clone(), 5, 8222, 6619, 6, Attack::Bludgeoning, 12);
    g5.add_immune(Attack::Fire);
    g5.add_immune(Attack::Slashing);

//    550 units each with 5068 hit points with an attack that does 87 radiation damage at initiative 19
    let g6 = Group::new(army.clone(), 6, 550, 5068, 87, Attack::Radiation, 19);

//    950 units each with 8681 hit points (weak to radiation) with an attack that does 73 slashing damage at initiative 17
    let mut g7 = Group::new(army.clone(), 7,950, 8681, 73, Attack::Slashing, 17);
    g7.add_weakness(Attack::Radiation);

//    28 units each with 9835 hit points with an attack that does 2979 bludgeoning damage at initiative 3
    let g8 = Group::new(army.clone(), 8, 28, 9835, 2979, Attack::Bludgeoning, 3);

//    3799 units each with 2933 hit points with an attack that does 7 slashing damage at initiative 16
    let g9 = Group::new(army.clone(), 9, 3799, 2933, 7, Attack::Slashing, 16);

//    35 units each with 8999 hit points (weak to bludgeoning; immune to radiation) with an attack that does 2505 cold damage at initiative 6
    let mut g10 = Group::new(army.clone(), 10, 35, 8999, 2505, Attack::Cold, 6);
    g10.add_weakness(Attack::Bludgeoning);
    g10.add_immune(Attack::Radiation);

    let mut immune = Army{groups: Vec::new(), name: army.clone()};
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

fn read_infection_large() -> Army {
    let army = "Infection".to_owned();

//    1639 units each with 28720 hit points with an attack that does 27 cold damage at initiative 8
    let g1 = Group::new(army.clone(), 1, 1639, 28720, 27, Attack::Cold, 8);

//    4968 units each with 16609 hit points (immune to slashing, bludgeoning, radiation) with an attack that does 6 fire damage at initiative 2
    let mut g2 = Group::new(army.clone(), 2, 4968, 16609, 6, Attack::Fire, 2);
    g2.add_immune(Attack::Slashing);
    g2.add_immune(Attack::Bludgeoning);
    g2.add_immune(Attack::Radiation);

//    3148 units each with 48970 hit points (weak to fire, bludgeoning) with an attack that does 29 slashing damage at initiative 20
    let mut g3 = Group::new(army.clone(), 3, 3148, 48970, 29, Attack::Slashing, 20);
    g3.add_weakness(Attack::Fire);
    g3.add_weakness(Attack::Bludgeoning);

//    1706 units each with 30069 hit points (immune to cold, bludgeoning) with an attack that does 29 fire damage at initiative 7
    let mut g4 = Group::new(army.clone(), 4, 1706, 30069, 29, Attack::Fire, 7);
    g4.add_immune(Attack::Cold);
    g4.add_immune(Attack::Bludgeoning);

//    496 units each with 39909 hit points (immune to cold; weak to radiation) with an attack that does 133 bludgeoning damage at initiative 4
    let mut g5 = Group::new(army.clone(), 5, 496, 39909, 133, Attack::Bludgeoning, 4);
    g5.add_immune(Attack::Cold);
    g5.add_weakness(Attack::Radiation);

//    358 units each with 17475 hit points with an attack that does 82 bludgeoning damage at initiative 5
    let g6 = Group::new(army.clone(), 6, 358, 17475, 82, Attack::Bludgeoning, 5);

//    120 units each with 53629 hit points with an attack that does 807 fire damage at initiative 15
    let g7 = Group::new(army.clone(), 7, 120, 53629, 807, Attack::Fire, 15);

//    402 units each with 44102 hit points (weak to slashing) with an attack that does 185 bludgeoning damage at initiative 14
    let mut g8 = Group::new(army.clone(), 8, 402, 44102, 185, Attack::Bludgeoning, 14);
    g8.add_weakness(Attack::Slashing);

//    468 units each with 11284 hit points (weak to fire) with an attack that does 43 radiation damage at initiative 18
    let mut g9 = Group::new(army.clone(), 9, 468, 11284, 43, Attack::Radiation, 18);
    g9.add_weakness(Attack::Fire);

//    4090 units each with 23075 hit points (immune to radiation) with an attack that does 10 bludgeoning damage at initiative 9
    let mut g10 = Group::new(army.clone(), 10, 4090, 23075, 10, Attack::Bludgeoning, 9);
    g10.add_immune(Attack::Radiation);

    let mut infection = Army{groups: Vec::new(), name: army.clone()};
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

struct Sim {
    immune: Army,
    infection: Army
}

impl Sim {
    fn select_targets(&mut self) {
        self.infection.select_targets(&self.immune);
        self.immune.select_targets(&self.infection);
    }

    fn attack(&mut self) {
        let mut prio_queue = Vec::new();
        for group in self.infection.groups.iter() {
            prio_queue.push(group);
        }
        for group in self.immune.groups.iter() {
            prio_queue.push(group);
        }

        prio_queue.sort_by(|&a, &b| b.initiative.cmp(&a.initiative));

        for group in prio_queue {
            if group.selected_enemy_id.get().is_none() {
                println!("{} group {} cannot attack anybody", group.army, group.id);
                continue;
            }
            if !group.alive.get() {
                continue;
            }

            let mut enemy_army: &Army;
            if group.army == self.immune.name {
                enemy_army = &self.infection;
            } else {
                enemy_army = &self.immune;
            }

            let selected_enemy_id = group.selected_enemy_id.get().unwrap();
            let enemy_group = enemy_army.get_group(selected_enemy_id).unwrap();

            let damage = group.attack_damage_to(enemy_group);
            let mut num_units_reduce = damage / enemy_group.hit_points_per_unit;

            if num_units_reduce >= enemy_group.num_units.get() {
                enemy_group.alive.set(false);
                num_units_reduce = enemy_group.num_units.get();
                enemy_group.num_units.set(0);
            } else {
                enemy_group.num_units.set(enemy_group.num_units.get() - num_units_reduce);
            }

            println!("{} group {} attacks defending group {}, killing {} units.  Target is alive: {}", group.army, group.id, enemy_group.id, num_units_reduce, enemy_group.alive.get());
        }

        println!();

    }
}

impl fmt::Debug for Sim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_owned();

        ret.push_str(&format!("{:?}", self.immune)[..]);
        ret.push_str(&format!("{:?}", self.infection)[..]);

        return write!(f, "{}", ret);
    }
}



#[derive(Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Army {
    name: String,
    groups: Vec<Group>
}

impl Army {
    fn select_targets(&mut self, other: &Army) {
        self.sort_groups();

        let mut taken_targets = Vec::new();

        for group in self.groups.iter() {
            if !group.alive.get() {
                continue;
            }

            let mut max_damage = 0;
            let mut max_enemys = Vec::new();

            for enemy_group in other.groups.iter() {
                if !enemy_group.alive.get() {
                    continue;
                }
                if taken_targets.contains(&enemy_group.id) {
                    continue;
                }

                let potential_damage =  group.attack_damage_to(enemy_group);
                println!("{} group {} would deal defending group {} {} damage", self.name, group.id, enemy_group.id, potential_damage);

                if potential_damage == 0 {
                    continue;
                }

                if potential_damage > max_damage {
                    max_enemys.clear();
                    max_damage = potential_damage;
                    max_enemys.push(enemy_group);
                } else if potential_damage == max_damage {
                    max_enemys.push(enemy_group);
                }
            }

            if max_enemys.len() == 0 {
                // If you cannot deal any damage, don't
                group.selected_enemy_id.set(None);
            } else {
                //If an attacking group is considering two defending groups to which it would deal equal damage,
                // it chooses to target the defending group with the largest effective power;
                // if there is still a tie, it chooses the defending group with the highest initiative

                // I can assume that we go through the full set of checks.  If there is only one left at some earlier
                // stage, there will only be one entry to compare to itself for max...

                let mut max_power = 0;
                let mut max_power_enemys = Vec::new();

                let effective_powers: Vec<isize> = max_enemys.iter().map(|&g| g.get_effective_power()).collect();
                for i in 0 .. effective_powers.len() {
                    let power = effective_powers[i];
                    if power > max_power {
                        max_power_enemys.clear();
                        max_power = power;
                        max_power_enemys.push(max_enemys[i]);
                    } else if power == max_power {
                        max_power_enemys.push(max_enemys[i]);
                    }
                }


                let mut max_initiative = 0;
                let mut max_initiative_enemy = None;

                let initiatives: Vec<isize> = max_power_enemys.iter().map(|&g| g.initiative).collect();
                for i in 0 .. initiatives.len() {
                    let initiative = initiatives[i];
                    if initiative > max_initiative {
                        max_initiative = initiative;
                        max_initiative_enemy = Some(max_power_enemys[i]);
                    }
                }

                let selected_enemy_id = max_initiative_enemy.unwrap().id;
                group.selected_enemy_id.set(Some(selected_enemy_id));
                taken_targets.push(selected_enemy_id);
            }
//            println!("{} group {}  selects enemy {:?}", self.name, group.id, group.selected_enemy_idx.get());
        }
    }


    fn sort_groups(&mut self) {
        self.groups.sort_by(|a, b| {
            let a_power = a.get_effective_power();
            let b_power = b.get_effective_power();
            if a_power == b_power {
                return b.initiative.cmp(&a.initiative);
            } else {
                return b_power.cmp(&a_power);
            }
        });
    }

    fn get_group(&self, id: isize) -> Option<&Group> {
        for group in self.groups.iter() {
            if group.id == id {
                return Some(group);
            }
        }
        return None;
    }

    fn has_alive(&self) -> bool {
        return self.num_alive() != 0;
    }

    fn num_alive(&self) -> isize {
        let mut ret = 0;
        for g in self.groups.iter() {
            if g.alive.get() {
                ret += g.num_units.get();
            }
        }

        return ret;
    }
}

impl fmt::Debug for Army {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = self.name.clone();

        ret.push_str(&"\n");

        for group in &self.groups {
            if !group.alive.get() {
                continue;
            }
            ret.push_str(&format!("{:?}\n", group)[..]);
        }

        return write!(f, "{}", ret);
    }
}

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Group {
    id: isize,
    army: String,
    num_units: Cell<isize>,
    hit_points_per_unit: isize,
    attack_damage_per_unit: isize,
    attack_type: Attack,
    initiative: isize,
    immunes: Vec<Attack>,
    weaknesses: Vec<Attack>,
    alive: Cell<bool>,
    selected_enemy_id: Cell<Option<isize>>
}

impl Group {
    fn new(army: String, id: isize, num_units: isize, hit_points_per_unit: isize, attack_damage_per_unit: isize, attack_type: Attack, initiative: isize) -> Group {
        return Group{army, id, num_units: Cell::new(num_units), hit_points_per_unit, attack_damage_per_unit, initiative, immunes: Vec::new(), weaknesses: Vec::new(), attack_type, alive: Cell::new(true), selected_enemy_id: Cell::new(None)}
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

    fn attack_damage_to(&self, enemy: &Group) -> isize {
        let mut damage = self.get_effective_power();

        let attack_type = self.attack_type;

        if enemy.immunes.contains(&attack_type) {
            damage = 0;
        }

        if enemy.weaknesses.contains(&attack_type) {
            damage *= 2;
        }

        return damage;
    }
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{} group {}. {} units.  {} hit-points per unit.   Initiative: {}    Effective power: {}   Damage: {}", self.army, self.id, self.num_units.get(), self.hit_points_per_unit, self.initiative, self.get_effective_power(), self.attack_damage_per_unit);
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

