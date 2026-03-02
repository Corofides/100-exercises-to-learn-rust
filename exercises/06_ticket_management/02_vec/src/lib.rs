// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
static mut FIB_NUMBERS: Vec<u32> = vec![];

pub struct Fib {
}

pub fn fibonacci(n: u32) -> u32 {

    let mut fib_numbers: Vec<u32> = vec![0, 1];
    
    let n = n as usize;
    let index = n + 1;

    if fib_numbers.len() >= index {
        return *fib_numbers.get(n).unwrap();
    }

    for index in fib_numbers.len() .. n + 1 {
        let mut fib_number = fib_numbers[index - 2] + fib_numbers[index - 1];
        fib_numbers.push(fib_number);
    }

    *fib_numbers.get(n).unwrap()

}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
