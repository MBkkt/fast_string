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
    for _n in 1..1000 {
        let x: u8 = random();
        let s = random_string(x as usize);
        let fs = FastString::new(s.as_str());
        assert_eq!(s, fs);
        assert_eq!(s.len(), fs.len());
        assert_eq!(s.is_empty(), fs.is_empty());
    }
}

#[test]
fn test_push() {
    for _n in 1..1000 {
        let mut x: u8 = random();
        let mut s = random_string(x as usize);
        let mut fs = FastString::new(s.as_str());
        x = random();
        let text = random_string(x as usize);
        for char in text.chars() {
            s.push(char);
            fs.push(char);
            assert_eq!(s, fs);
            assert_eq!(s.len(), fs.len());
            assert_eq!(s.is_empty(), fs.is_empty());
        }
    }
}

#[test]
fn test_push_str() {
    for _n in 1..1000 {
        let mut x: u8 = random();
        let mut s = random_string(x as usize);
        let mut fs = FastString::new(s.as_str());
        x = random();
        let text = random_string(x as usize);
        s.push_str(text.as_str());
        fs.push_str(text.as_str());
        assert_eq!(s, fs);
        assert_eq!(s.len(), fs.len());
        assert_eq!(s.is_empty(), fs.is_empty());
    }
}


#[test]
fn test_clone() {
    for _n in 1..1000 {
        let mut x: u8 = random();
        let mut s = random_string(x as usize);
        let mut fs = FastString::new(s.as_str());
        let mut fs_clone = fs.clone();
        x = random();
        let text = random_string(x as usize);
        for char in text.chars() {
            s.push(char);
            fs.push(char);
            fs_clone.push(char);
            assert_eq!(s, fs);
            assert_eq!(s.len(), fs.len());
            assert_eq!(s.is_empty(), fs.is_empty());
            assert_eq!(fs, fs_clone);
            assert_eq!(fs.len(), fs_clone.len());
            assert_eq!(fs.is_empty(), fs_clone.is_empty());
        }
    }
}


#[test]
fn test_clone2() {
    for _n in 1..1000 {
        let mut x: u8 = random();
        let mut s = random_string(x as usize);
        let s_clone = s.clone();
        let mut fs = FastString::new(s.as_str());
        let fs_clone = fs.clone();
        x = random();
        let text = random_string(x as usize);
        for char in text.chars() {
            s.push(char);
            fs.push(char);
            assert_eq!(s, fs);
            assert_eq!(s.len(), fs.len());
            assert_eq!(s.is_empty(), fs.is_empty());
            assert_eq!(s_clone, fs_clone);
            assert_eq!(s_clone.len(), fs_clone.len());
            assert_eq!(s_clone.is_empty(), fs_clone.is_empty());
        }
    }
}

