use fast_string::FastString;
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use rand::{thread_rng, Rng};
use std::iter;
use std::time::{Duration, SystemTime};

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

////////////////////////////////////////////////////////////////////////////////////////////////////
enum BenchType {
    Small,
    Medium,
    Large,
}

fn print_bench_result(bench_type: BenchType, name: &str, s_time_ns: f64, fs_time_ns: f64) {
    print!("bench_{} for str ", name);
    match bench_type {
        BenchType::Small => println!("smaller than 24 byte"),
        BenchType::Medium => println!("with size like 1Kb"),
        BenchType::Large => println!("with size like 1Mb"),
    }
    if s_time_ns < fs_time_ns {
        println!("std::String faster than FastString");
    } else {
        println!("FastString faster than std::String");
    }
    println!(
        "FastString {} % of the time std::String",
        fs_time_ns / s_time_ns * 100.0
    );
    println!("std::String time ms: {}", s_time_ns / 1000_000.0);
    println!("FastString time ms: {}\n", fs_time_ns / 1000_000.0);
}

#[test]
fn bench_from_small() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);
    for _ in 0..1000 {
        let x: usize = random();
        let example = random_string(x % 23 + 1);

        let s_start = SystemTime::now();
        let s = String::from(example.as_str());
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        let fs = FastString::from(example.as_str());
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s, &fs));
    }
    print_bench_result(
        BenchType::Small,
        "from",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

#[test]
fn bench_from_medium() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);
    for _ in 0..1000 {
        let x: usize = random();
        let example = random_string(x % 1024 + 1024);

        let s_start = SystemTime::now();
        let s = String::from(example.as_str());
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        let fs = FastString::from(example.as_str());
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s, &fs));
    }
    print_bench_result(
        BenchType::Medium,
        "from",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

#[test]
fn bench_from_large() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    let mut x: usize = random();
    let mut example: String = random_string(x % 1024 + 1024 * 1023);

    for i in 0..1000 {
        if i % 100 == 0 {
            x = random();
            example = random_string(x % 1024 + 1024 * 1023);
        }
        let s_start = SystemTime::now();
        let s = String::from(example.as_str());
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        let fs = FastString::from(example.as_str());
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s, &fs));
    }
    print_bench_result(
        BenchType::Large,
        "from",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn bench_clone_small() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);
    for _ in 0..1000 {
        let x: usize = random();
        let s_example = random_string(x % 23 + 1);
        let fs_example = FastString::from(s_example.as_str());
        assert!(is_same(&s_example, &fs_example));

        let s_start = SystemTime::now();
        let s = s_example.clone();
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        let fs = fs_example.clone();
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s, &fs));
    }
    print_bench_result(
        BenchType::Small,
        "clone",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

#[test]
fn bench_clone_medium() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);
    for _ in 0..1000 {
        let x: usize = random();
        let s_example = random_string(x % 1024 + 1024);
        let fs_example = FastString::from(s_example.as_str());
        assert!(is_same(&s_example, &fs_example));

        let s_start = SystemTime::now();
        let s = s_example.clone();
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        let fs = fs_example.clone();
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s, &fs));
    }
    print_bench_result(
        BenchType::Medium,
        "clone",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

#[test]
fn bench_clone_large() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    let mut x: usize;
    let mut s_example = String::new();
    let mut fs_example = FastString::new();

    for i in 0..1000 {
        if i % 100 == 0 {
            x = random();
            s_example = random_string(x % 1024 + 1024 * 1023);
            fs_example = FastString::from(s_example.as_str());
            assert!(is_same(&s_example, &fs_example));
        }
        let s_start = SystemTime::now();
        let s = s_example.clone();
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        let fs = fs_example.clone();
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s, &fs));
    }
    print_bench_result(
        BenchType::Large,
        "clone",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn bench_push_small() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);
    for _ in 0..1000 {
        let x: usize = random();
        let mut s_example = random_string(x % 23 + 1);
        let mut fs_example = FastString::from(s_example.as_str());
        let ch: char = random();
        assert!(is_same(&s_example, &fs_example));

        let s_start = SystemTime::now();
        s_example.push(ch);
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        fs_example.push(ch);
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(
        BenchType::Small,
        "push",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

#[test]
fn bench_push_medium() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);
    for _ in 0..1000 {
        let x: usize = random();
        let mut s_example = random_string(x % 1024 + 1024);
        let mut fs_example = FastString::from(s_example.as_str());
        let ch: char = random();
        assert!(is_same(&s_example, &fs_example));

        let s_start = SystemTime::now();
        s_example.push(ch);
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        fs_example.push(ch);
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(
        BenchType::Medium,
        "push",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

#[test]
fn bench_push_large() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    let mut x: usize;
    let mut s_example = String::new();
    let mut fs_example = FastString::new();

    for i in 0..1000 {
        if i % 100 == 0 {
            x = random();
            s_example = random_string(x % 1024 + 1024 * 1023);
            fs_example = FastString::from(s_example.as_str());
            assert!(is_same(&s_example, &fs_example));
        }
        let ch: char = random();

        let s_start = SystemTime::now();
        s_example.push(ch);
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        fs_example.push(ch);
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(
        BenchType::Large,
        "push",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn bench_remove_small() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);
    for _ in 0..1000 {
        let mut x: usize = random();
        let mut s_example = random_string(x % 23 + 1);
        let mut fs_example = FastString::from(s_example.as_str());
        x = random();
        let mut idx = x % s_example.len();
        while !s_example.is_char_boundary(idx) {
            x = random();
            idx = x % s_example.len();
        }
        assert!(is_same(&s_example, &fs_example));

        let s_start = SystemTime::now();
        s_example.remove(idx);
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        fs_example.remove(idx);
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(
        BenchType::Small,
        "remove",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

#[test]
fn bench_remove_medium() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);
    for _ in 0..1000 {
        let mut x: usize = random();
        let mut s_example = random_string(x % 1024 + 1024);
        let mut fs_example = FastString::from(s_example.as_str());
        x = random();
        let mut idx = x % s_example.len();
        while !s_example.is_char_boundary(idx) {
            x = random();
            idx = x % s_example.len();
        }
        assert!(is_same(&s_example, &fs_example));

        let s_start = SystemTime::now();
        s_example.remove(idx);
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        fs_example.remove(idx);
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(
        BenchType::Medium,
        "remove",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

#[test]
fn bench_remove_large() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    let mut x: usize;
    let mut s_example = String::new();
    let mut fs_example = FastString::new();

    for i in 0..1000 {
        if i % 100 == 0 {
            x = random();
            s_example = random_string(x % 1024 + 1024 * 1023);
            fs_example = FastString::from(s_example.as_str());
            assert!(is_same(&s_example, &fs_example));
        }
        x = random();
        let mut idx = x % s_example.len();
        while !s_example.is_char_boundary(idx) {
            x = random();
            idx = x % s_example.len();
        }
        assert!(is_same(&s_example, &fs_example));

        let s_start = SystemTime::now();
        s_example.remove(idx);
        s_time += s_start.elapsed().expect("");

        let fs_start = SystemTime::now();
        fs_example.remove(idx);
        fs_time += fs_start.elapsed().expect("");

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(
        BenchType::Large,
        "remove",
        s_time.as_nanos() as f64,
        fs_time.as_nanos() as f64,
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////
