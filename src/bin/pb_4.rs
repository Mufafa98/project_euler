fn is_palindrom(n: u32) -> bool {
    let mut digits: Vec<u32> = Vec::new();
    let mut n = n;
    while n != 0 {
        let digit = n % 10;
        n /= 10;

        digits.push(digit);
    }
    let before = digits.clone();
    digits.reverse();
    return before == digits;
}

fn main() {
    let mut largest = 0;

    for d1 in 100..1000 {
        for d2 in d1..1000 {
            if is_palindrom(d1 * d2) {
                if d1 * d2 > largest {
                    largest = d1 * d2;
                }
            }
        }
    }

    println!("{}", largest);
}
