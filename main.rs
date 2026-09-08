use std::io::{self, BufRead};
use std::rc::{self, Rc};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let rc_nums = Rc::new(nums.clone());
    let _handle1 = Rc::clone(&rc_nums);
    let _handle2 = Rc::clone(&rc_nums);

    println!("count: {}", Rc::strong_count(&rc_nums));
    println!("sum: {}", nums.iter().sum::<i32>());
}
