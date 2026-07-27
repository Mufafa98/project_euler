fn main() {
    let mut divisor = 2;
    let mut largest_factor = divisor;
    let mut n: u64 = 600851475143;
    while n > 1 {
        while n % divisor == 0 {
            largest_factor = divisor;
            n /= divisor;
        }
        divisor += 1;
    }
    println!("{}", largest_factor);
}
