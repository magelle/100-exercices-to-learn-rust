// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let (first, second) = slice.split_at(slice.len() / 2);

    let first_thread = thread::spawn(|| first.iter().sum::<i32>());

    let second_thread = thread::spawn(|| second.iter().sum::<i32>());

    first_thread.join().unwrap() + second_thread.join().unwrap()
}
