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

pub fn uniform<T>(vals: Vec<T>) -> Distribution<T> {
    // use std::convert::TryFrom;
    // let len = isize::try_from(vals.len()).unwrap();
    use super::util;
    let len = util::to_isize(vals.len()).expect("Given vector too long");
    let mut result = Vec::new();
    for v in vals {
        let val = v;
        let chance = Rational::new(1, len);
        result.push(Event { val, chance });
    }
    Distribution(result)
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

    pub fn apply<F, U>(&self, fs: Distribution<F>) -> Distribution<U>
    where
        F: Fn(&T) -> U,
    {
        let mut result = Vec::new();
        for v in self.0.iter() {
            for f in &fs.0 {
                let val = (f.val)(&v.val);
                let chance = f.chance * v.chance;
                result.push(Event { val, chance });
            }
        }
        Distribution(result)
    }
}