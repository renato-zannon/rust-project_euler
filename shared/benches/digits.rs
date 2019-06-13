use self::criterion::*;
use shared::digits::{self, Digits};

fn bench(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "digits",
        |b: &mut Bencher, n: &u64| {
            b.iter(|| {
                let mut digits: Digits<_, u64> = digits::new(*n);
                loop {
                    match digits.next() {
                        Some(_) => {}
                        None => break,
                    }
                }
            });
        },
        vec![0, 12, 145, 12890, 87660, 9998989898, 131234123401023840],
    );
}

criterion_group!(benches, bench);
criterion_main!(benches);
