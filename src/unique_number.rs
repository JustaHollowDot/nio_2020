use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{Read, stdin};

struct UniqueNumber;
impl UniqueNumber {
    pub fn run() {
        let mut contents = String::new();
        let _ = stdin().read_to_string(&mut contents);

        let (_, contents) = contents.split_once("\n").unwrap();

        let mut numbers_amount = BinaryHeap::new();

        for line in contents.lines() {
            let number: i128 = line.parse().unwrap();

            numbers_amount.push(Reverse(number));
        }

        while let Some(current) = numbers_amount.pop() {
            if current == numbers_amount.pop().unwrap() {
                while let Some(next) = numbers_amount.peek() {
                    if current != *next {
                        break
                    } else {
                        numbers_amount.pop();
                    }

                }
            } else {
                println!("{}", current.0);
                return
            }
        }

        println!("-1");
    }
}

pub fn main() {
    UniqueNumber::run();
}