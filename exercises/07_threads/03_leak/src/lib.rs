// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
   
    let static_v: &'static mut [i32] = v.leak();


    let l_handle = {

        let static_v = &static_v[..];

        thread::spawn(move || {
            static_v[0..(static_v.len() / 2)].iter()
                .fold(0, |acc, x| x + acc)
        })
    };

    let r_handle = {

        let static_v = &static_v[..];

        thread::spawn(move || {
            static_v[(static_v.len() / 2)..static_v.len()].iter()
                .fold(0, |acc, x| x + acc)
        })

    };

    let mut result = l_handle.join().unwrap();
    result += r_handle.join().unwrap();
    result
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
