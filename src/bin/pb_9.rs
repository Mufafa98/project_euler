// TODO: Optimize

fn main() {
    for a in 1..1000 {
        for b in a..1000 {
            let c = 1000 - a - b;
            if c * c == a * a + b * b {
                println!("a: {} b: {} c: {} prod: {}", a, b, c, a * b * c);
            }
        }
    }
}
