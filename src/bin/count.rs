fn main() {
    let w = std::env::args()
        .nth(1)
        .expect("missing size")
        .parse()
        .unwrap();
    let b = std::env::args()
        .nth(2)
        .map(|s| s.parse().unwrap())
        .unwrap_or(2);
    let d = based::Iter::new(w, b);
    for i in d {
        println!("{}", i);
    }
}
