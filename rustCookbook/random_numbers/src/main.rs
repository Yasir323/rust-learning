use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    println!("Random u8: {}", n1);
}
