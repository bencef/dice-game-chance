use num_rational::Ratio;

pub mod distribution {

    use num_rational::Ratio;

    pub struct Event<T> {
        val: T,
        chance: Ratio,
    }
}

fn main() {
    println!("{}!", Ratio::new(1, 2));
}
