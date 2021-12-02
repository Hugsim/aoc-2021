use crate::util::*;

use std::str::FromStr;

#[derive(Clone)]
enum Direction {
    Forward,
    Down,
    Up
}
impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "forward" {
            Ok(Direction::Forward)
        } else if s == "down" {
            Ok(Direction::Down)
        } else if s == "up" {
            Ok(Direction::Up)
        } else {
            Err(())
        }
    }
}

#[derive(Clone)]
struct Instruction (Direction, i32);
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut strs = s.split(" ");
        let dir = strs.next().unwrap().unsafe_parse::<Direction>();
        let amnt = strs.next().unwrap().unsafe_parse::<i32>();

        Ok(Instruction(dir, amnt))
    }
}

pub fn solve() -> (Option<i32>, Option<i32>) {
    let contents = read_file_to_vec::<Instruction>("src/days/input/2");

    let mut distance_forward_1 = 0;
    let mut distance_down_1 = 0;

    for instr in contents.clone() {
        match instr {
            Instruction(Direction::Forward, a) => {
                distance_forward_1 += a;
            },
            Instruction(Direction::Down, a) => {
                distance_down_1 += a;
            },
            Instruction(Direction::Up, a) => {
                distance_down_1 -= a;
            }
        }
    }

    let mut distance_forward_2 = 0;
    let mut distance_down_2 = 0;
    let mut aim = 0;

    for instr in contents {
        match instr {
            Instruction(Direction::Forward, a) => {
                distance_forward_2 += a;
                distance_down_2 += aim * a;
            },
            Instruction(Direction::Down, a) => {
                aim += a;
            },
            Instruction(Direction::Up, a) => {
                aim -= a;
            }
        }
    }

    (Some(distance_down_1 * distance_forward_1), Some(distance_down_2 * distance_forward_2))
}