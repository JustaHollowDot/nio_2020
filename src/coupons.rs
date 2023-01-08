use std::io::{Read, stdin};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Coupon {
    discount: i32,
    min_amount: i32,
    min_amount_with_discount: i32,
}

impl Ord for Coupon {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min_amount_with_discount.cmp(&other.min_amount_with_discount)
    }
}

impl PartialOrd for Coupon {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Coupons;
impl Coupons {
    pub fn run() {
        let mut input = String::new();
        let _ = stdin().read_to_string(&mut input).unwrap();

        dbg!(&input);

        let mut lines = input.lines();

        let (price_to_pay, _) = lines.next().unwrap().split_once(" ").unwrap();
        let price_to_pay = price_to_pay.parse::<i32>().unwrap();

        dbg!(price_to_pay);

        let mut coupons = Vec::new();

        for line in lines {
            let (discount, min_amount) = line.split_once(" ").unwrap();
            let discount = discount.parse::<i32>().unwrap();
            let min_amount = min_amount.parse::<i32>().unwrap();
            let min_amount_with_discount = min_amount + discount;

            coupons.push(Coupon {
                discount,
                min_amount,
                min_amount_with_discount,
            });
        }

        coupons.sort();

        let mut min_price = price_to_pay;

        while coupons.len() > 0 {
            let first_coupon = coupons[0].clone();
            coupons.remove(0);

            let mut current_price = price_to_pay;

            if current_price >= first_coupon.min_amount_with_discount {
                dbg!(&coupons);

                for coupon in coupons.iter() {
                    dbg!(current_price);
                    dbg!(coupon);

                    if current_price >= coupon.min_amount_with_discount {
                        current_price -= coupon.discount;
                    }
                }
            }

            dbg!(current_price);

            if current_price < min_price {
                min_price = current_price;
            }
        }

        println!("{}", min_price);
    }
}

pub fn main() {
    Coupons::run();
}