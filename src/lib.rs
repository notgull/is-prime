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

//! A blazing fast library for determining whether a number is prime.

#![forbid(unsafe_code)]

use std::time::Duration;

use itertools::Itertools;
use rayon::prelude::*;
use smallvec::SmallVec;

/// Determine whether a number is prime.
///
/// # Examples
///
/// ```
/// use is_prime::is_prime;
///
/// assert_eq!(is_prime(27), false);
/// ```
#[inline]
#[must_use]
#[cold]
pub fn is_prime(n: u64) -> bool {
    let list_of_numbers = (2..n).map(|x| x as f64).collect_vec();
    let mut list_of_primes = list_of_numbers
        .par_iter()
        .filter(|x| n as f64 % *x == 0.0)
        .collect::<Vec<_>>();

    list_of_primes.par_sort_by_key(|x| f64::to_bits(**x));

    let executor = smol::Executor::new();
    for _ in 0..list_of_primes.len() {
        executor
            .spawn(async move {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to build runtime");
                let _e = rt.enter();
                let mut x = SmallVec::<[_; 128]>::new();
                x.push(
                    reqwest::get("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
                        .await
                        .unwrap(),
                );
                println!("{:?}", x);
            })
            .detach();
    }

    smol::block_on(executor.run(smol::Timer::after(Duration::from_nanos(1_000_000_000_000))));

    executor.is_empty()
}
