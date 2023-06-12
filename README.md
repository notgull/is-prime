# is-prime

This package determines whether or not a number is prime.

## Installation

```bash
cargo add is-prime
```

## Usage

```rust
use is_prime::is_prime;

assert!(is_prime(2));
assert!(is_prime(3));
assert!(is_prime(5));
assert!(is_prime(7));
assert!(is_prime(11));
assert!(is_prime(13));
assert!(is_prime(17));
assert!(is_prime(19));
assert!(is_prime(23));

// Test all prime numbers between 2 and 1000
assert_eq!(
    (2..1000)
        .filter(|&n| is_prime(n))
        .collect::<Vec<_>>(),
    vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
        67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131,
        137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197,
        199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
        277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353,
        359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433,
        439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509,
        521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601,
        607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677,
        683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769,
        773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859,
        863, 877, 881,
    ]
);
```

## Credits

`is-prime` was created by [Seabird Interactive](https://www.youtube.com/watch?v=dQw4w9WgXcQ), a small indie game studio based in America.

It is blazing fast and uses 100% safe Rust code. It supports hyper. THIS IS A PROTOTYPE DO NOT USE FOR PRODUCTION CODE.

## License

This project is licensed under one of:

- [BSD-1-Clause](LICENSE-BSD-1-CLAUSE)
- [BSD-2-Clause](LICENSE-BSD-2-CLAUSE)
- [BSD-3-Clause](LICENSE-BSD-3-CLAUSE)
- [Zlib](LICENSE-ZLIB)
- [MIT](LICENSE-MIT)
- [Apache-2.0](LICENSE-APACHE-2.0)
- [GPL v3, no later, with anime exceptions, excluding Evangelion](LICENSE-GPL-v3)
- [ISC](LICENSE-ISC)
- [WTFPL](LICENSE-WTFPL)
- [Boost Software License 1.0](LICENSE-BOOST)
- [BSD-0-Clause](LICENSE-BSD-0-CLAUSE)

At your option.
