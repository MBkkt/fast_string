use fast_string::FastString;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

mod bench;
mod test;

fn random_string(n: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(n)
        .collect()
}

fn is_same(s: &String, fs: &FastString) -> bool {
    s.is_empty() == fs.is_empty() && s.len() == fs.len() && s == fs
}
