use std::{
    collections::HashMap,
    io::{self, BufRead},
    ops::{Add, Mul, Sub, SubAssign},
    str::FromStr,
};

use lazy_static::lazy_static;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

// cannot implement Ord or PartialOrd, it's not antisymmetric
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
struct Amount {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u8,
    ore_robot_cost: Amount,
    clay_robot_cost: Amount,
    obsidian_robot_cost: Amount,
    geode_robot_cost: Amount,
    max_geodes: HashMap<(usize, Amount, Amount), usize>, // (time, robots, materials) => max geodes
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.$").unwrap();
        }

        let cap = RE.captures(s).unwrap();
        Ok(Blueprint {
            id: cap[1].parse().unwrap(),
            ore_robot_cost: Amount {
                ore: cap[2].parse().unwrap(),
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay_robot_cost: Amount {
                ore: cap[3].parse().unwrap(),
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian_robot_cost: Amount {
                ore: cap[4].parse().unwrap(),
                clay: cap[5].parse().unwrap(),
                obsidian: 0,
                geode: 0,
            },
            geode_robot_cost: Amount {
                ore: cap[6].parse().unwrap(),
                clay: 0,
                obsidian: cap[7].parse().unwrap(),
                geode: 0,
            },
            max_geodes: HashMap::new(),
        })
    }
}

fn read_input() -> Vec<Blueprint> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|ln| Blueprint::from_str(&ln).unwrap())
        .collect()
}

impl Material {
    fn get(&self, amount: &Amount) -> usize {
        amount.get(*self)
    }
}

impl Blueprint {
    fn max_cost(&self, material: Material) -> usize {
        [
            &self.ore_robot_cost,
            &self.clay_robot_cost,
            &self.obsidian_robot_cost,
            &self.geode_robot_cost,
        ]
        .iter()
        .map(|cost| material.get(cost))
        .max()
        .unwrap()
    }
}

impl Amount {
    fn can_afford(&self, other: &Self) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    fn ore(value: usize) -> Self {
        Amount {
            ore: value,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn clay(value: usize) -> Self {
        Amount {
            ore: 0,
            clay: value,
            obsidian: 0,
            geode: 0,
        }
    }

    fn obsidian(value: usize) -> Self {
        Amount {
            ore: 0,
            clay: 0,
            obsidian: value,
            geode: 0,
        }
    }

    fn geodes(value: usize) -> Self {
        Amount {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: value,
        }
    }

    fn get(&self, material: Material) -> usize {
        match material {
            Material::Ore => self.ore,
            Material::Clay => self.clay,
            Material::Obsidian => self.obsidian,
            Material::Geode => self.geode,
        }
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}

impl Sub for Amount {
    type Output = Amount;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.can_afford(&rhs));
        Amount {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl Add for Amount {
    type Output = Amount;

    fn add(self, rhs: Self) -> Self::Output {
        Amount {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Mul<usize> for Amount {
    type Output = Amount;

    fn mul(self, rhs: usize) -> Self::Output {
        Amount {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

fn has_enough_for_robots(blueprint: &Blueprint, robots: Amount) -> bool {
    robots.can_afford(&blueprint.ore_robot_cost)
        && robots.can_afford(&blueprint.clay_robot_cost)
        && robots.can_afford(&blueprint.obsidian_robot_cost)
        && robots.can_afford(&blueprint.geode_robot_cost)
}

fn needs_more_miners(
    blueprint: &Blueprint,
    time: usize,
    robots: Amount,
    materials: Amount,
    material: Material,
) -> bool {
    materials.get(material) + (robots.get(material) * time) < time * blueprint.max_cost(material)
}

const PRIORITISE_GEODE_MINERS: bool = true;

fn max_geodes(blueprint: &mut Blueprint, time: usize, robots: Amount, materials: Amount) -> usize {
    if time == 0 {
        return materials.geode;
    }
    let cache_key = (time, robots, materials);
    if let Some(cached_max) = blueprint.max_geodes.get(&cache_key) {
        return *cached_max;
    }

    // without building new robot this turn
    let next_materials = materials.add(robots);
    let mut max = if materials.geode > 0 {
        // without building robots at all?
        // may be wrong optimization
        materials.geode + (robots.geode * time)
    } else {
        max_geodes(blueprint, time - 1, robots, next_materials)
    };

    // don't care, build a geode miner
    if materials.can_afford(&blueprint.geode_robot_cost) {
        max = max.max(max_geodes(
            blueprint,
            time - 1,
            robots.add(Amount::geodes(1)),
            next_materials.sub(blueprint.geode_robot_cost),
        ));
        if PRIORITISE_GEODE_MINERS {
            blueprint.max_geodes.insert(cache_key, max);
            return max;
        }
    }

    // if any robot can be built, check if it's better to build it
    // if production >= any robot, only build geode robots
    let should_build_non_geode = !has_enough_for_robots(blueprint, robots);
    if should_build_non_geode {
        if materials.can_afford(&blueprint.ore_robot_cost)
            && needs_more_miners(blueprint, time, robots, materials, Material::Ore)
        {
            max = max.max(max_geodes(
                blueprint,
                time - 1,
                robots.add(Amount::ore(1)),
                next_materials.sub(blueprint.ore_robot_cost),
            ));
        }
        if materials.can_afford(&blueprint.clay_robot_cost)
            && needs_more_miners(blueprint, time, robots, materials, Material::Clay)
        {
            max = max.max(max_geodes(
                blueprint,
                time - 1,
                robots.add(Amount::clay(1)),
                next_materials.sub(blueprint.clay_robot_cost),
            ));
        }
        if materials.can_afford(&blueprint.obsidian_robot_cost)
            && needs_more_miners(blueprint, time, robots, materials, Material::Obsidian)
        {
            max = max.max(max_geodes(
                blueprint,
                time - 1,
                robots.add(Amount::obsidian(1)),
                next_materials.sub(blueprint.obsidian_robot_cost),
            ));
        }
    }

    blueprint.max_geodes.insert(cache_key, max);
    max
}

fn part1(blueprints: &Vec<Blueprint>) -> usize {
    blueprints
        .par_iter()
        .map(|b| {
            max_geodes(&mut b.clone(), 24, Amount::ore(1), Amount::default()) * (b.id as usize)
        })
        .sum()
}

fn part2(blueprints: &Vec<Blueprint>) -> usize {
    blueprints.as_slice()[0..blueprints.len().min(3)]
        .par_iter()
        .map(|b| max_geodes(&mut b.clone(), 32, Amount::ore(1), Amount::default()))
        .reduce(|| 1, |acc, b| acc * b)
}

fn main() {
    let blueprints = read_input();
    println!("part 1: {}", part1(&blueprints));
    println!("part 2: {}", part2(&blueprints));
}
