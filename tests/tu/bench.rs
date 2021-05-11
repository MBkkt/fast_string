use crate::is_same;
use crate::random_string;
use fast_string::FastString;
use rand::prelude::*;
use std::time::{Duration, Instant};

////////////////////////////////////////////////////////////////////////////////////////////////////
enum BenchType {
    Small,
    Medium,
    Large,
}

fn get_iter(bench_type: BenchType) -> usize {
    match bench_type {
        BenchType::Small => 100_000,
        BenchType::Medium => 10_000,
        BenchType::Large => 1_000,
    }
}

fn print_bench_result(bench_type: BenchType, name: &str, s_time: Duration, fs_time: Duration) {
    print!("bench_{} for str ", name);
    match bench_type {
        BenchType::Small => println!("smaller than 24 byte"),
        BenchType::Medium => println!("with size like 1Kb"),
        BenchType::Large => println!("with size like 1Mb"),
    }
    if s_time < fs_time {
        println!("std::String faster than FastString");
    } else {
        println!("FastString faster than std::String");
    }
    println!(
        "FastString {} % of the time std::String",
        fs_time.as_nanos() as f64 / s_time.as_nanos() as f64 * 100.0
    );
    println!("std::String time: {:?}", s_time);
    println!("FastString  time: {:?}", fs_time);
}

#[test]
fn bench_from_small() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    for _ in 0..get_iter(BenchType::Small) {
        let x: usize = random();
        let example = random_string(x % 23 + 1);

        let s_start = Instant::now();
        let s = String::from(example.as_str());
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        let fs = FastString::from(example.as_str());
        fs_time += fs_start.elapsed();

        assert!(is_same(&s, &fs));
    }
    print_bench_result(BenchType::Small, "from", s_time, fs_time);
}

#[test]
fn bench_from_medium() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    for _ in 0..get_iter(BenchType::Medium) {
        let x: usize = random();
        let example = random_string(x % 1024 + 1024);

        let s_start = Instant::now();
        let s = String::from(example.as_str());
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        let fs = FastString::from(example.as_str());
        fs_time += fs_start.elapsed();

        assert!(is_same(&s, &fs));
    }
    print_bench_result(BenchType::Medium, "from", s_time, fs_time);
}

#[test]
fn bench_from_large() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    let mut x: usize = random();
    let mut example: String = random_string(x % 1024 + 1024 * 1023);

    for i in 0..get_iter(BenchType::Large) {
        if i % 100 == 0 {
            x = random();
            example = random_string(x % 1024 + 1024 * 1023);
        }
        let s_start = Instant::now();
        let s = String::from(example.as_str());
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        let fs = FastString::from(example.as_str());
        fs_time += fs_start.elapsed();

        assert!(is_same(&s, &fs));
    }
    print_bench_result(BenchType::Large, "from", s_time, fs_time);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn bench_clone_small() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    for _ in 0..get_iter(BenchType::Small) {
        let x: usize = random();
        let s_example = random_string(x % 23 + 1);
        let fs_example = FastString::from(s_example.as_str());
        assert!(is_same(&s_example, &fs_example));

        let s_start = Instant::now();
        let s = s_example.clone();
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        let fs = fs_example.clone();
        fs_time += fs_start.elapsed();

        assert!(is_same(&s, &fs));
    }
    print_bench_result(BenchType::Small, "clone", s_time, fs_time);
}

#[test]
fn bench_clone_medium() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    for _ in 0..get_iter(BenchType::Medium) {
        let x: usize = random();
        let s_example = random_string(x % 1024 + 1024);
        let fs_example = FastString::from(s_example.as_str());
        assert!(is_same(&s_example, &fs_example));

        let s_start = Instant::now();
        let s = s_example.clone();
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        let fs = fs_example.clone();
        fs_time += fs_start.elapsed();

        assert!(is_same(&s, &fs));
    }
    print_bench_result(BenchType::Medium, "clone", s_time, fs_time);
}

#[test]
fn bench_clone_large() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    let mut x: usize;
    let mut s_example = String::new();
    let mut fs_example = FastString::new();

    for i in 0..get_iter(BenchType::Large) {
        if i % 100 == 0 {
            x = random();
            s_example = random_string(x % 1024 + 1024 * 1023);
            fs_example = FastString::from(s_example.as_str());
            assert!(is_same(&s_example, &fs_example));
        }
        let s_start = Instant::now();
        let s = s_example.clone();
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        let fs = fs_example.clone();
        fs_time += fs_start.elapsed();

        assert!(is_same(&s, &fs));
    }
    print_bench_result(BenchType::Large, "clone", s_time, fs_time);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn bench_push_small() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    for _ in 0..get_iter(BenchType::Small) {
        let x: usize = random();
        let mut s_example = random_string(x % 23 + 1);
        let mut fs_example = FastString::from(s_example.as_str());
        let ch: char = random();
        assert!(is_same(&s_example, &fs_example));

        let s_start = Instant::now();
        s_example.push(ch);
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        fs_example.push(ch);
        fs_time += fs_start.elapsed();

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(BenchType::Small, "push", s_time, fs_time);
}

#[test]
fn bench_push_medium() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    for _ in 0..get_iter(BenchType::Medium) {
        let x: usize = random();
        let mut s_example = random_string(x % 1024 + 1024);
        let mut fs_example = FastString::from(s_example.as_str());
        let ch: char = random();
        assert!(is_same(&s_example, &fs_example));

        let s_start = Instant::now();
        s_example.push(ch);
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        fs_example.push(ch);
        fs_time += fs_start.elapsed();

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(BenchType::Medium, "push", s_time, fs_time);
}

#[test]
fn bench_push_large() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    let mut x: usize;
    let mut s_example = String::new();
    let mut fs_example = FastString::new();

    for i in 0..get_iter(BenchType::Large) {
        if i % 100 == 0 {
            x = random();
            s_example = random_string(x % 1024 + 1024 * 1023);
            fs_example = FastString::from(s_example.as_str());
            assert!(is_same(&s_example, &fs_example));
        }
        let ch: char = random();

        let s_start = Instant::now();
        s_example.push(ch);
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        fs_example.push(ch);
        fs_time += fs_start.elapsed();

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(BenchType::Large, "push", s_time, fs_time);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn bench_remove_small() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    for _ in 0..get_iter(BenchType::Small) {
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

        let s_start = Instant::now();
        s_example.remove(idx);
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        fs_example.remove(idx);
        fs_time += fs_start.elapsed();

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(BenchType::Small, "remove", s_time, fs_time);
}

#[test]
fn bench_remove_medium() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    for _ in 0..get_iter(BenchType::Medium) {
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

        let s_start = Instant::now();
        s_example.remove(idx);
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        fs_example.remove(idx);
        fs_time += fs_start.elapsed();

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(BenchType::Medium, "remove", s_time, fs_time);
}

#[test]
fn bench_remove_large() {
    let mut s_time = Duration::from_nanos(0);
    let mut fs_time = Duration::from_nanos(0);

    let mut x: usize;
    let mut s_example = String::new();
    let mut fs_example = FastString::new();

    for i in 0..get_iter(BenchType::Large) {
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

        let s_start = Instant::now();
        s_example.remove(idx);
        s_time += s_start.elapsed();

        let fs_start = Instant::now();
        fs_example.remove(idx);
        fs_time += fs_start.elapsed();

        assert!(is_same(&s_example, &fs_example));
    }
    print_bench_result(BenchType::Large, "remove", s_time, fs_time);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
