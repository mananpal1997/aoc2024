use std::io::{self, Write};

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

    let seconds = 10000;
    for second in 0..seconds {
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
        println!("{}", second+1);
        print_grid(g_height, g_width, &robots);
    }

    0
}

fn print_grid(g_height: i64, g_width: i64, robots: &Vec<Robot>) {
    let mut grid: Vec<Vec<String>> = Vec::new();

    for _ in 0..g_height {
        let mut row: Vec<String> = Vec::new();
        for _ in 0..g_width {
            row.push(".".to_string());
        }
        grid.push(row);
    }

    for r in robots {
        grid[r.pos_y as usize][r.pos_x as usize] = "#".to_string();
    }

    let mut frame = String::new();
    frame.push_str("\x1B[2J\x1B[H");

    for row in grid {
        for col in row {
            frame.push_str(col.as_str());
        }
        frame.push_str("\n");
    }

    println!("{}", frame);
    io::stdout().flush().unwrap();
}