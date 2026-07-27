fn main() {
    let mut counter = 0;
    let mut prime = 0;
    let mut number = 2;
    while counter < 10_001 {
        let mut is_prime = true;
        if number % 2 == 0 && number != 2 {
            is_prime = false;
        }
        let mut divisor = 3;
        while divisor * divisor < number {
            if number % divisor == 0 {
                is_prime = false;
            }
            divisor += 2;
        }
        if is_prime {
            prime = number;
            counter += 1;
        }
        number += 1;
    }

    println!("{}", prime);
}
