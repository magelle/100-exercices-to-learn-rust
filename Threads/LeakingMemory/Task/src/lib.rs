// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let leaked = v.leak();
    let mid = leaked.len() / 2;
    let (first, last) = leaked.split_at(mid);

    let firstSum = thread::spawn(|| first.iter().sum::<i32>());
    let lastSum = thread::spawn(|| last.iter().sum::<i32>());

    firstSum.join().unwrap() + lastSum.join().unwrap()
}
