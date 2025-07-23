// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    if v.len() == 0 {
        return 0;
    }
    let v_leaked: &'static mut [i32] = v.leak();
    // let half_count = v_leaked.len() / 2;
    let t1 = thread::spawn( || {
        let half_count = v_leaked.len() / 2;
        v_leaked[..half_count].iter().sum()
    });
    let t2 = thread::spawn( || {
        let half_count = v_leaked.len() / 2;
        v_leaked[half_count..].iter().sum()
    });

    let r1: i32 = t1.join().expect("msg");
    let r2: i32 = t2.join().expect("msg");
    r1 + r2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
