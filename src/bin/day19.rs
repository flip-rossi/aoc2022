//! Day 19: Not Enough Minerals

use aoc22::line_reader::LineReader;

const TIME: i32 = 24;

/// Each robot's cost is a tuple where (ore, clay, obsidian)
#[derive(Debug)]
struct Blueprint {
    ore_robot: (i32,i32,i32),
    clay_robot: (i32,i32,i32),
    obsidian_robot: (i32,i32,i32),
    geode_robot: (i32,i32,i32),
}

fn main() {
    let mut blueprints = Vec::new();
    // Parse input
    let mut lr = LineReader::new();
    while lr.read_next().unwrap() > 0 {
        let nums = lr.as_numbers(10);
        blueprints.push(Blueprint {
            ore_robot: (nums[1] as i32, 0, 0),
            clay_robot: (nums[2] as i32, 0, 0),
            obsidian_robot: (nums[3] as i32, nums[4] as i32, 0),
            geode_robot: (nums[5] as i32, 0, nums[6] as i32)
        });
    }
    eprintln!("{blueprints:?}");

    // Solve
    let answer = aoc22::solve_puzzle!(blueprints);
    println!("{answer}")
}

enum Robot {
    Ore, Clay, Obsidian, Geode
}

//=============== PART 1 ===============//
fn factory_start_building(blueprint: &Blueprint, minutes_left: i32, robots: (i32,i32,i32,i32), resources: (i32,i32,i32,i32)) -> i32 {
    let mut geode_build = 0;
    let mut obsidian_build = 0;
    let mut clay_build = 0;
    let mut ore_build = 0;
    // Start building the new robots
    if resources.0 >= blueprint.geode_robot.0 && resources.1 >= blueprint.geode_robot.1 && resources.2 >= blueprint.geode_robot.2 {
        let timeline_resources = (
            resources.0 - blueprint.geode_robot.0,
            resources.1 - blueprint.geode_robot.1,
            resources.2 - blueprint.geode_robot.2,
            resources.3,
        );
        geode_build = factory_collect_and_build(blueprint, minutes_left, robots, timeline_resources, Some(Robot::Geode));
    }
    if resources.0 >= blueprint.obsidian_robot.0 && resources.1 >= blueprint.obsidian_robot.1 && resources.2 >= blueprint.obsidian_robot.2 {
        let timeline_resources = (
            resources.0 - blueprint.obsidian_robot.0,
            resources.1 - blueprint.obsidian_robot.1,
            resources.2 - blueprint.obsidian_robot.2,
            resources.3,
        );
        obsidian_build = factory_collect_and_build(blueprint, minutes_left, robots, timeline_resources, Some(Robot::Obsidian));
    }
    if resources.0 >= blueprint.clay_robot.0 && resources.1 >= blueprint.clay_robot.1 && resources.2 >= blueprint.clay_robot.2 {
        let timeline_resources = (
            resources.0 - blueprint.clay_robot.0,
            resources.1 - blueprint.clay_robot.1,
            resources.2 - blueprint.clay_robot.2,
            resources.3,
        );
        clay_build = factory_collect_and_build(blueprint, minutes_left, robots, timeline_resources, Some(Robot::Clay));
    }
    if resources.0 >= blueprint.ore_robot.0 && resources.1 >= blueprint.ore_robot.1 && resources.2 >= blueprint.ore_robot.2 {
        let timeline_resources = (
            resources.0 - blueprint.ore_robot.0,
            resources.1 - blueprint.ore_robot.1,
            resources.2 - blueprint.ore_robot.2,
            resources.3,
        );
        ore_build = factory_collect_and_build(blueprint, minutes_left, robots, timeline_resources, Some(Robot::Ore));
    }
    let no_build = factory_collect_and_build(blueprint, minutes_left, robots, resources, None);

    let a = geode_build.max(obsidian_build).max(clay_build).max(ore_build).max(no_build);
    // eprintln!("{a}");
    a
}

fn factory_collect_and_build(blueprint: &Blueprint, minutes_left: i32, mut robots: (i32,i32,i32,i32), mut resources: (i32,i32,i32,i32), to_build: Option<Robot>) -> i32 {
    // Collect resources
    resources.0 += robots.0;
    resources.1 += robots.1;
    resources.2 += robots.2;
    resources.3 += robots.3;

    // Finish building the robot
    if let Some(r) = to_build {
        match r {
            Robot::Ore => robots.0 += 1,
            Robot::Clay => robots.1 += 1,
            Robot::Obsidian => robots.2 += 1,
            Robot::Geode => robots.3 += 1,
        }
    }

    // Start next minute
    if minutes_left > 0 {
        factory_start_building(blueprint, minutes_left-1, robots, resources)
    }
    else {
        // eprintln!("Resources: {resources:?}\nRobots: {robots:?}");
        resources.3
    }
}

fn part1(blueprints: Vec<Blueprint>) -> i32 {
    let mut quality_sum = 0;
    for (i, bp) in blueprints.iter().enumerate() {
        let most_geodes = factory_start_building(bp, TIME, (1,0,0,0), (0,0,0,0));
        quality_sum += (i+1) as i32 * most_geodes;
        eprintln!("{i}+1 * {most_geodes} = {}", i+1);
    }
    quality_sum
}

//=============== PART 2 ===============//
#[allow(dead_code, unused_variables)]
fn part2(blueprints: Vec<Blueprint>) -> ! {
    todo!()
}

