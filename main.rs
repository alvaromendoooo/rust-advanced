use std::io::{self, BufRead};
use std::rc::{self, Rc};
use std::cell::RefCell;

fn main() {
    // Test 01
    /*let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let rc_nums = Rc::new(nums.clone());
    let _handle1 = Rc::clone(&rc_nums);
    let _handle2 = Rc::clone(&rc_nums);

    println!("count: {}", Rc::strong_count(&rc_nums));
    println!("sum: {}", nums.iter().sum::<i32>());*/

    // Test 02
    let counter = Rc::new(RefCell::new(0));

    for _ in 0..3 {
        let handle = Rc::clone(&counter);
        *handle.borrow_mut() += 1;
    }

    println!("{}", counter.borrow());
}
