use super::distribution as dist;

pub fn die(sides: u8) -> dist::Distribution<u8> {
    use std::iter::FromIterator;
    dist::uniform(Vec::from_iter(1..=sides))
}

// TODO: use a macro to make `plus' optional
pub fn d(amount: u8, sides: u8, plus: u8) -> dist::Distribution<u8> {
    let mut result = dist::always(0);
    for _ in 0..amount {
        result = summing(result, die(sides));
    }
    result.map(|orig| orig + plus).normalized()
}

fn summing(
    orig_throws: dist::Distribution<u8>,
    new_throws: dist::Distribution<u8>,
) -> dist::Distribution<u8> {
    fn add_two(orig: &u8) -> impl Fn(&u8) -> u8 {
        let orig = orig.clone();
        move |new| orig + new
    }
    new_throws.apply(orig_throws.map(add_two))
}
