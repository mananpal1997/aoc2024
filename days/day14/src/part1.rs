use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
struct Robot {
    pos_x: i64,
    pos_y: i64,
    v_x: i64,
    v_y: i64
}

pub fn solve(_input: &str) -> i64 {
    let mut lines = _input.lines();
    let (mut g_width, mut g_height) = (-1, -1);
    match lines.nth(0).unwrap().split_once(",") {
        Some((gw, gh)) => {
            g_width = gw.parse::<i64>().unwrap();
            g_height = gh.parse::<i64>().unwrap();
        },
        None => {}
    }

    let fobidden_x = g_width / 2;
    let forbidden_y = g_height / 2;

    let mut robots: Vec<Robot> = lines.map(|line| {
        let (position, velocity) = line.trim().split_once(" ").unwrap();
        let (_, coords) = position.split_once("=").unwrap();
        let (coord_x, coord_y) = coords.split_once(",").unwrap();
        let (_, velocities) = velocity.split_once("=").unwrap();
        let (vel_x, vel_y) = velocities.split_once(",").unwrap();

        let robot = Robot{
            pos_x: coord_x.parse::<i64>().unwrap(),
            pos_y: coord_y.parse::<i64>().unwrap(),
            v_x: vel_x.parse::<i64>().unwrap(),
            v_y: vel_y.parse::<i64>().unwrap(),
        };
        return robot;
    }).collect();

    let seconds = 100;
    for _ in 0..seconds {
        for r in &mut robots {
            r.pos_x += r.v_x;
            r.pos_y += r.v_y;

            while r.pos_x < 0 {
                r.pos_x += g_width;
            }
            r.pos_x %= g_width;
            while r.pos_y < 0 {
                r.pos_y += g_height;
            }
            r.pos_y %= g_height;
        }
    }

    let mut quadrants = [0, 0, 0, 0];
    for robot in robots {
        if robot.pos_x == fobidden_x || robot.pos_y == forbidden_y {
            continue;
        }
        match (robot.pos_x.cmp(&fobidden_x), robot.pos_y.cmp(&forbidden_y)) {
            (Ordering::Less, Ordering::Less) => {quadrants[0]+=1;},
            (Ordering::Greater, Ordering::Less) => {quadrants[1]+=1;},
            (Ordering::Less, Ordering::Greater) => {quadrants[2]+=1;},
            (Ordering::Greater, Ordering::Greater) => {quadrants[3]+=1;},
            _ => {}
        }
    }

    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}