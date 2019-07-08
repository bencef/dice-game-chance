mod dice;
mod distribution;
mod util;

fn collect(first: &u8) -> impl Fn(&u8) -> [u8; 2] {
    let first = first.clone();
    move |second| [first, second.clone()]
}

fn compare(first: &bool) -> impl Fn(&bool) -> bool {
    let first = first.clone();
    move |second| first && *second
}

fn main() {
    use self::dice;
    let d6 = dice::d(1, 6, 0);
    let collect_throws = d6.map(collect);
    let two_throws = d6.apply(collect_throws);
    let sixty_five = two_throws
        .map(|[x, y]| (*x == 6 && *y == 5) || (*x == 5 && *y == 6))
        .normalized();
    let bas = two_throws.map(|[x, y]| *x == *y).normalized();
    let bas_after_sixty_five = bas.apply(sixty_five.map(compare)).normalized();

    println!("Chance of sixty-five: {:?}", sixty_five);
    println!("Chance of bas: {:?}", bas);
    println!("Chance of bas after sixty-five: {:?}", bas_after_sixty_five);
}
