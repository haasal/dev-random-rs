use random::Rng;

fn main() {
    let mut rng = Rng::with_cap(15);
    let buf = rng.next_bytes();
    println!("{buf:?}")
}
