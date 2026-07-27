pub fn pb_1() {
    let n = 1000;
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("{}", sum)
}
