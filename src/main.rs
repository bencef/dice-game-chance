mod util;
mod distribution;
mod dice;

fn main() {
    use self::dice;
    println!("2d4+3: {:?}", dice::d(2, 4, 3));
}
