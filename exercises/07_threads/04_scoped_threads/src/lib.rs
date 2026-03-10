// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    let midpoint = v.len() / 2;
    let mut value = 0;

    std::thread::scope(|scope| {
        let handle_1 = scope.spawn(|| {
            v[..midpoint].iter()
                .fold(0, |acc, x| acc + x)
        });
        let handle_2 = scope.spawn(|| {
            v[midpoint..].iter()
                .fold(0, |acc, x| acc + x)
        });

        let s1 = handle_1.join().unwrap();
        let s2 = handle_2.join().unwrap();

        s1 + s2
    })
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
