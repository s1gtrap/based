use based::UniqDigits;

fn main() {
    let n = std::env::args()
        .nth(1)
        .expect("missing size")
        .parse()
        .unwrap();
    let d = UniqDigits::new(2, n);
    for d in d {
        println!(
            "{} + {}",
            d,
            d.clone()
                .skip(1)
                .take_while(|d| !d.1.contains(&(d.0 - 1)))
                .count()
                + 1,
        );
    }
}
