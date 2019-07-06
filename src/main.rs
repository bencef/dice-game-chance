use num_rational::Ratio;

pub mod distribution {

    use num_rational::Rational;

    pub struct Event<T> {
        val: T,
        chance: Rational,
    }

    pub type Distribution<T> = Vec<Event<T>>;

    pub fn always<T>(val: T) -> Distribution<T> {
        let chance = Rational::new(1, 1);
        vec![Event { val, chance }]
    }

    pub fn map<T, U>(f: &Fn(&T) -> U, vals: &Distribution<T>) -> Distribution<U> {
        let mut result = Vec::new();
        for v in vals {
            result.push(Event {
                val: f(&v.val),
                chance: v.chance,
            });
        }
        result
    }
}

fn main() {
    println!("{}!", Ratio::new(1, 2));
}
