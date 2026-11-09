// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread::scope;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let mut sum = 0;

    let (first, second) = v.split_at(mid);

    scope(|scope| {
        let first_sum = scope.spawn(move || first.iter().sum::<i32>());
        let second_sum = scope.spawn(move || second.iter().sum::<i32>());
        first_sum.join().unwrap() + second_sum.join().unwrap()
    })
}
