fn normal_sum_squared(n: u32) -> u32 {
    let sum = n * (n + 1) / 2;
    sum * sum
}

fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

fn main() {
    let n = 100;
    let normal_sum = normal_sum_squared(n);
    let squared_sum = sum_of_squares(n);
    let diff = normal_sum - squared_sum;
    println!("{} {} {}", normal_sum, squared_sum, diff);
}
