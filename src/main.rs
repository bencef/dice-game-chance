pub mod distribution {

    use num_rational::Rational;

    #[derive(Debug)]
    struct Event<T> {
        val: T,
        chance: Rational,
    }

    #[derive(Debug)]
    pub struct Distribution<T>(Vec<Event<T>>);

    pub fn always<T>(val: T) -> Distribution<T> {
        let chance = Rational::new(1, 1);
        Distribution(vec![Event { val, chance }])
    }

    impl<T> Distribution<T> {
        pub fn map<F, U>(&self, f: F) -> Distribution<U>
        where
            F: Fn(&T) -> U,
        {
            let mut result = Vec::new();
            for v in self.0.iter() {
                let val = f(&v.val);
                let chance = v.chance.clone();
                result.push(Event { val, chance });
            }
            Distribution(result)
        }
    }
}

fn main() {
    use self::distribution as dist;
    let answer = dist::always(42);
    let check = |x: &u32| 42 == *x;
    println!("{:?}", answer.map(check));
    println!("answer was: {:?}", answer);
}
