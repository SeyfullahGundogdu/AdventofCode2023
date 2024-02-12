
//greatest common divisor
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

//least common multiple
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

//least common multiple for an array of numbers
pub fn total_lcm(numbers: &[usize]) -> usize {
    numbers.iter().cloned().fold(1, lcm)
}