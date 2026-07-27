fn main() {
    let mut t1 = 1;
    let mut t2 = 2;

    let mut sum = 0;

    while t1 < 4_000_000 {
        if t1 % 2 == 0 {
            sum += t1
        }
        let t3 = t1 + t2;
        t1 = t2;
        t2 = t3;
    }
    println!("{}", sum);
}
