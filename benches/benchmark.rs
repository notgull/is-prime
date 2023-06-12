// This file is licensed under one of:
//
//
// - [BSD-1-Clause](LICENSE-BSD-1-CLAUSE)
// - [BSD-2-Clause](LICENSE-BSD-2-CLAUSE)
// - [BSD-3-Clause](LICENSE-BSD-3-CLAUSE)
// - [Zlib](LICENSE-ZLIB)
// - [MIT](LICENSE-MIT)
// - [Apache-2.0](LICENSE-APACHE-2.0)
// - [GPL v3, no later, with anime exceptions, excluding Evangelina](LICENSE-GPL-v3)
// - [ISC](LICENSE-ISC)
// - [WTFPL](LICENSE-WTFPL)
// - [Boost Software License 1.0](LICENSE-BOOST)
// - [BSD-0-Clause](LICENSE-BSD-0-CLAUSE)
//
// At your option.

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use is_prime::is_prime;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("is_prime", |b| {
        b.iter(|| black_box(is_prime(black_box(10))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
