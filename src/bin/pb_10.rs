fn old_main() {
    let mut primes: Vec<u64> = (2..2_000_000).collect();

    let mut sum = 0;
    while !primes.is_empty() {
        let num = primes[0];
        sum += num;
        primes = primes.into_iter().filter(|x| x % num != 0).collect();
    }
    println!("{}", sum);
}

fn main() {
    const LIMIT: usize = 2_000_000;

    // Remove all even numbers
    const SIEVE_BOUND: usize = (LIMIT - 1) / 2;
    // Cross only until sqrt(LIMIT)
    // Divided by 2 because of halfing the LIMIT
    let cross_limit = (((LIMIT as f64).sqrt().floor() - 1.0) / 2.0) as usize;

    let mut sieve = [false; SIEVE_BOUND];
    for i in 1..cross_limit {
        if !sieve[i] {
            let mut j = 2 * i * (i + 1);
            while j < SIEVE_BOUND {
                sieve[j] = true;
                j += 2 * i + 1;
            }
        }
    }
    let mut sum = 2;
    for i in 0..SIEVE_BOUND {
        if !sieve[i] {
            sum += 2 * i + 1;
        }
    }

    println!("{}", sum);
}
