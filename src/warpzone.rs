use std::collections::VecDeque;
use std::io::{Read, stdin};

#[derive(Debug)]
struct Stage {
    jump_points: Vec<usize>,
    visited: bool,
}

#[derive(Debug)]
struct State {
    stage_index: usize,
    cost: usize,
}

struct WarpZone;
impl WarpZone {
    pub fn run() {
        let mut input = String::new();
        let _ = stdin().read_to_string(&mut input).unwrap();

        dbg!(&input);

        let mut lines = input.lines();

        let first_line = lines.next().unwrap();
        let stages_amount = first_line.parse::<u32>().unwrap();

        let mut stages = Vec::new();

        dbg!(lines.clone().count());

        stages.push(Stage {
            jump_points: vec![],
            visited: false,
        });

        for line in lines {
            let stage_jump_points = line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

            let stage = Stage {
                jump_points: stage_jump_points,
                visited: false,
            };

            stages.push(stage);
        }

        dbg!(stages_amount);
        dbg!(&stages);

        let mut queue = VecDeque::new();

        queue.push_back(State {
            stage_index: 0,
            cost: 0
        });

        while let Some(current_stage) = queue.pop_front() {
            dbg!(&current_stage);
            dbg!(&stages[current_stage.stage_index]);

            if current_stage.stage_index + 1 == stages_amount as usize {
                println!("{}", current_stage.cost + 1);
                return;
            }

            for jump_point in stages[current_stage.stage_index].jump_points.clone().iter() {
                if *jump_point == stages_amount as usize {
                    println!("{}", current_stage.cost + 1);
                    return;
                }

                if !stages[*jump_point].visited {
                    stages[*jump_point].visited = true;

                    queue.push_back(State {
                        stage_index: *jump_point,
                        cost: current_stage.cost + 1
                    });
                }
            }



            if !stages[current_stage.stage_index + 1].visited {
                if current_stage.stage_index + 1 == stages_amount as usize {
                    println!("{}", current_stage.cost + 1);
                    return;
                }

                stages[current_stage.stage_index + 1].visited = true;

                queue.push_back(State {
                    stage_index: current_stage.stage_index + 1,
                    cost: current_stage.cost + 1
                });
            }
        }

        println!("{}", stages_amount);
    }
}

pub fn main() {
    WarpZone::run();
}