use std::io::{Read, stdin};

#[derive(Eq, PartialEq, Debug)]
struct State {
    value: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct AccountingFraud;
impl AccountingFraud {
    pub fn run() {
        let mut input = String::new();
        let _ = stdin().read_to_string(&mut input).unwrap();

        dbg!(&input);

        let mut lines = input.lines();

        lines.next();

        let mut current_value = 0;

        let mut lowest_values = std::collections::BinaryHeap::new();

        let mut transactions_removed = 0;

        for line in lines {
            let transaction = line.parse::<i32>().unwrap();

            dbg!(transaction);

            if transaction < 0 {
                lowest_values.push(State {
                    value: transaction,
                });
            }

            current_value += transaction;

            while current_value < 0 {
                let lowest_value = lowest_values.pop().unwrap();

                current_value -= lowest_value.value;

                transactions_removed += 1;
            }

            dbg!(&current_value);
            dbg!(&transactions_removed);
        }

        println!("{}", transactions_removed);
    }
}

pub fn main() {
    AccountingFraud::run();
}