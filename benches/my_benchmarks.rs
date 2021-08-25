use criterion::{criterion_group, criterion_main, Criterion};
use leto;

fn criterion_benchmark(c: &mut Criterion) {
    let input = String::from("Hello, Wor").repeat(100_000_000);
    let input_ref = input.as_bytes();
    c.bench_function(
        "Rust string to char",
        |b| b.iter(|| {
            let _y = input.chars().all(|x| x == 'a');
        }),
    );
    c.bench_function(
        "Rust split",
        |b| b.iter(|| {
            let _y = input.split(' ').last();
        }),
    );
    c.bench_function(
        "LeTo byte2char",
        |b| b.iter(|| {
            let _y = leto::byte2char::Byte2Char::new(input_ref).all(|x| x == 'a');
        }),
    );

    c.bench_function(
        "LeTo char2token with byte2char",
        |b| b.iter(|| {
            let y:std::cell::Cell<String> = std::cell::Cell::default();
            let _z = leto::char2token::Char2Token::new(leto::byte2char::Byte2Char::new(input_ref), &y).last();
        }),
    );

    c.bench_function(
        "LeTo char2token with rust string to char",
        |b| b.iter(|| {
            let y:std::cell::Cell<String> = std::cell::Cell::default();
            let _z = leto::char2token::Char2Token::new(input.chars(), &y).last();
        }),
    );
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);