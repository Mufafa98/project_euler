pub fn pb_0() {
    let n = 442_000;
    let mut sum: u128 = 0;
    for i in 0..n {
        let num = i + 1;
        if num % 2 == 0 {
            continue;
        }
        sum += num * num;
    }
    println!("{}", sum);
}
