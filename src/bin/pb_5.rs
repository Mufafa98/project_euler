// TODO: Optimize
fn criterion(n: u64, max: u64) -> bool {
    let nums = 2..max;
    for num in nums {
        if n % num != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut number = 1;
    loop {
        if criterion(number, 20) {
            break;
        }
        number += 1;
    }
    println!("{}", number);
}
