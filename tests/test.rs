use fast_string::FastString;
use rand::prelude::*;
use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

fn random_string(n: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(n)
        .collect()
}

#[test]
fn test_simple() {
    for n in 1..100 {
        let x: u8 = random();
        let s = random_string(2 * (x as usize));
        let fs = FastString::new(s.as_str());
        assert_eq!(s, fs);
        println!("string number {}: size: {}, chars: {}", n, 2 * (x as usize), fs)
    }
}