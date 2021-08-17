use criterion::{criterion_group, criterion_main, Criterion};
use leto;
fn criterion_benchmark(c: &mut Criterion) {
    let input = String::from("Hello, World!").repeat(1000);
    let input_ref = input.as_bytes();

    c.bench_function(
        "Rust string to char",
        |b| b.iter(|| {

            //let _y:Vec<char> = input.chars().collect();
            let mut i = 0;
            for _y in input.chars() {
                i += 1;
            }
            assert_eq!(i, 13000);
        }),
    );

    c.bench_function(
        "LeTo byte2char",
        |b| b.iter(|| {
            
            //let _x:Vec<char> = leto::byte2char::Byte2Char::new(input_ref).collect();
            let mut i = 0;
            for _x in leto::byte2char::Byte2Char::new(input_ref) {
                i += 1;
            }
            assert_eq!(i, 13000);
        }),
    );
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);