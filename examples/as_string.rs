use random::Rng;

fn main() {
    let mut rng = Rng::with_cap(1000);
    rng.next_bytes();
    println!("{}", rng.as_string())
}
