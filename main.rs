use std::io::{self, BufRead};
use std::rc::Rc;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // TODO: move `nums` into an Rc.
    // TODO: create two more handles to it.
    // TODO: print "count: " and the strong count.
    // TODO: print "sum: " and the sum of the shared vec.
}
