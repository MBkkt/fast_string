use std::iter;

use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use rand::{thread_rng, Rng};

use fast_string::FastString;

fn random_string(n: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(n)
        .collect()
}

fn is_same(s: &String, fs: &FastString) -> bool {
    // Why not just `s == fs` ?
    s.is_empty() == fs.is_empty() && s.len() == fs.len() && s == fs
}

fn push_command(s: &mut String, fs: &mut FastString, ch: char) -> bool {
    s.push(ch);
    fs.push(ch);
    is_same(s, fs)
}

fn push_str_command(s: &mut String, fs: &mut FastString, add: &str) -> bool {
    s.push_str(add);
    fs.push_str(add);
    is_same(s, fs)
}

fn remove_command(s: &mut String, fs: &mut FastString, idx: usize) -> bool {
    if s.is_empty() {
        return true;
    }
    let index = idx % s;
    if !s.is_char_boundary(index) {
        return true;
    }
    s.remove(index);
    fs.remove(index);
    is_same(s, fs)
}

#[derive(Clone, Debug)]
enum Command {
    Push { ch: char },
    PushStr { add: String },
    Remove { idx: usize },
    Clone,
}

impl Arbitrary for Command {
    fn arbitrary(g: &mut Gen) -> Command {
        match g.choose(&[0, 1, 2, 3]) {
            Some(0) => Command::Push {
                ch: char::arbitrary(g),
            },
            Some(1) => Command::PushStr {
                add: String::arbitrary(g),
            },
            Some(2) => Command::Remove {
                idx: usize::arbitrary(g),
            },
            Some(3) => Command::Clone {},
            _ => unreachable!(),
        }
    }
}

#[quickcheck]
fn test_push_prop(mut string: String, ch: char) -> bool {
    let mut fast_string = FastString::new(string.as_str());
    push_command(&mut string, &mut fast_string, ch)
}

#[quickcheck]
fn test_push_str_prop(mut string: String, add: String) -> bool {
    let mut fast_string = FastString::new(string.as_str());
    push_str_command(&mut string, &mut fast_string, add.as_str())
}

#[quickcheck]
fn test_remove_prop(mut string: String, index: usize) -> bool {
    let mut fast_string = FastString::new(string.as_str());
    remove_command(&mut string, &mut fast_string, index)
}

#[quickcheck]
fn test_all_prop(mut string: String, commands: Vec<Command>) -> bool {
    let mut fast_string = FastString::new(string.as_str());
    let mut clones = Vec::new();

    for command in commands.iter() {
        // :thumbsup:
        let result = match command {
            Command::Push { ch } => push_command(&mut string, &mut fast_string, *ch),
            Command::PushStr { add } => {
                push_str_command(&mut string, &mut fast_string, add.as_str())
            }
            Command::Remove { idx } => remove_command(&mut string, &mut fast_string, *idx),
            Command::Clone => {
                // Do we want to assert some props about clones?
                clones.push(fast_string.clone());
                is_same(&string, &fast_string)
            }
        };
        if !result {
            return false;
        }
    }

    true
}

// Can we remove random tests? quickcheck-based tessts are probably more
// powerful now?
#[test]
fn test_simple() {
    for _ in 1..1000 {
        let x: u8 = random();
        let s = random_string(x as usize);
        let fs = FastString::new(s.as_str());
        assert!(is_same(&s, &fs));
    }
}

#[test]
fn test_push() {
    for _ in 1..1000 {
        let mut x: u8 = random();
        let mut s = random_string(x as usize);
        let mut fs = FastString::new(s.as_str());
        x = random();
        let add = random_string(x as usize);
        assert!(is_same(&s, &fs));
        for char in add.chars() {
            assert!(push_command(&mut s, &mut fs, char));
        }
    }
}

#[test]
fn test_push_str() {
    for _ in 1..1000 {
        let mut x: u8 = random();
        let mut s = random_string(x as usize);
        let mut fs = FastString::new(s.as_str());
        x = random();
        let add = random_string(x as usize);
        assert!(is_same(&s, &fs));
        assert!(push_str_command(&mut s, &mut fs, add.as_str()));
    }
}

#[test]
fn test_remove() {
    for _ in 1..1000 {
        let x: u8 = random();
        let mut s = random_string(x as usize);
        let mut fs = FastString::new(s.as_str());
        assert!(is_same(&s, &fs));
        while !fs.is_empty() {
            let index = random();
            assert!(remove_command(&mut s, &mut fs, index));
        }
    }
}

#[test]
fn test_clone() {
    for _n in 1..1000 {
        let mut x: u8 = random();
        let mut s = random_string(x as usize);
        let mut fs = FastString::new(s.as_str());
        let mut fs_clone = fs.clone();
        assert!(is_same(&s, &fs));
        assert!(is_same(&s, &fs_clone));
        x = random();
        let text = random_string(x as usize);
        for char in text.chars() {
            s.push(char);
            fs.push(char);
            fs_clone.push(char);
            assert!(is_same(&s, &fs));
            assert!(is_same(&s, &fs_clone));
        }
    }
}
