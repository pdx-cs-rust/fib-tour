//! # Rust Fibonacci Number Functions Tour
//! Bart Massey *et al* 2021
//!
//! The functions in this library compute the `n`-th Fibonacci
//! number, starting with `fibonacci_number(0)` = 0 and
//! `fibonacci_number(1)` = 1. They return `Some` if the result
//! fits in a `u32`, and `None` otherwise.

/// This is the function by <http://reddit.com/u/findingajobaccount>
/// in
/// [this Reddit thread](https://www.reddit.com/r/learnrust/comments/s17ldm/review_my_simple_fibonacci_function/)
/// that inspired this tour. I've taken the liberty of renaming,
/// cleaning up the types and interface, fixing Clippy warnings, and
/// fixing a couple of bugs.
pub fn fib_vec(n: usize) -> Option<u32> {
    if n == 0 {
        return Some(0);
    }
    if n >= 48 {
        return None;
    }
    let mut dp = vec![0u32, 1];
    for i in 2..=n {
        dp.push(dp.get(i - 2).unwrap() + dp.get(i - 1).unwrap());
    }
    return Some(*dp.last().unwrap());
}

/// This is `fib_vec` with some cleanups to make it a bit more
/// readable. Thanks to <http://reddit.com/u/YetiBarBar> for the
/// suggestion to use `with_capacity()`.
pub fn fib_vec_fancy(n: usize) -> Option<u32> {
    let mut dp = <Vec<u32>>::with_capacity(n);
    dp.extend(&[0, 1]);
    for i in 2..=n {
        dp.push(dp[i - 2].checked_add(dp[i - 1])?);
    }
    Some(dp[n])
}

/// The implementation from
/// [this comment](https://www.reddit.com/r/learnrust/comments/s17ldm/review_my_simple_fibonacci_function/hs733oo/)
/// by <https://www.reddit.com/user/yomand4847> uses a neat
/// array version of the more standard dynamic-programming-style
/// approach to computing Fibonacci Numbers. I've adapted the
/// interface, cleaned up Clippy warnings, and fixed a bug in
/// that the original could not safely compute F(47).
pub fn fib_array(n: usize) -> Option<u32> {
    if n == 0 {
        return Some(0);
    }
    let mut dp = [0u32, 1];
    for _ in 0..n - 1 {
        dp = [dp[1], dp[0].checked_add(dp[1])?];
    }
    Some(dp[1])
}

/// This unrolled version of `fib_array()` should be quite
/// fast. It takes advantage of the fact that, because
/// of the structure of F(n), anytime the high bit is set
/// that must be the last value.
pub fn fib_registered(n: usize) -> Option<u32> {
    let mut x = 0u32;
    let mut y = 1;
    let hi = !0 ^ (!0 >> 1);
    let mut m = n & !1;
    while m > 0 {
        if y & hi == 0 {
            x = x.wrapping_add(y);
        } else {
            break;
        }
        if x & hi == 0 {
            y = y.wrapping_add(x);
        } else {
            break;
        }
        m -= 2;
    }
    if m > 0 {
        None
    } else if n & 1 == 0 {
        Some(x)
    } else if x & hi == 0 {
        Some(y)
    } else {
        None
    }
}

/// A functional implementation can use `fold()`.
pub fn fib_fold(n: usize) -> Option<u32> {
    (0..n)
        .fold(Some((0u32, 1u32)), |regs, _| {
            regs.and_then(|(x, y)| u32::checked_add(x, y).map(|z| (z, x)))
        })
        .map(|(x, _)| x)
}

/// A fancy functional implementation can compute lazily.
/// This function could just return the iterator of
/// Fibonacci Numbers that it produces.
pub fn fib_lazy(n: usize) -> Option<u32> {
    let mut state = Some((0, 1));
    let advance = move || {
        state.map(|(x, y)| {
            state = u32::checked_add(x, y).map(|z| (z, x));
            x
        })
    };
    std::iter::from_fn(advance).nth(n)
}

/// This
/// [closed-form](https://en.wikipedia.org/wiki/Fibonacci_number#Closed-form_expression)
/// implementation is pretty fast. It works for `u32` but won't
/// work so well for bigger types because floating-point fail.
pub fn fib_closed(n: usize) -> Option<u32> {
    const SQRT5: f64 = 2.23606797749979;
    const PHI: f64 = (1.0 + SQRT5) / 2.0;
    const PSI: f64 = 1.0 - PHI;
    let f_approx = (PHI.powf(n as f64) - PSI.powf(n as f64)) / SQRT5;
    let f_rounded = (f_approx + 0.5).floor();
    // 2**32 - 1
    const MAX_FIB: f64 = 4294967295.0;
    if f_rounded <= MAX_FIB {
        Some(f_rounded as u32)
    } else {
        None
    }
}

/// This dumb implementation is what you do if you want
/// really, really fast Fibonacci Numbers.
pub fn fib_lookup(n: usize) -> Option<u32> {
    const F: &[u32] = &[
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
        2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
        165580141, 267914296, 433494437, 701408733, 1134903170, 1836311903, 2971215073,
    ];
    F.get(n).copied()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_fib(name: &str, f: fn(usize) -> Option<u32>) {
        let fibs = [0, 1, 1, 2, 3, 5, 8];
        for (i, fib) in fibs.into_iter().enumerate() {
            let ff = f(i);
            eprintln!("{}: {} {:?}", name, i, ff);
            assert_eq!(Some(fib), ff);
        }
        assert_eq!(Some(701_408_733), f(44));
        assert_eq!(Some(1_134_903_170), f(45));
        assert_eq!(Some(1_836_311_903), f(46));
        assert_eq!(Some(2_971_215_073), f(47));
        for i in 48..=51 {
            let ff = f(i);
            eprintln!("{}: {} {:?}", name, i, ff);
            assert!(ff.is_none());
        }
        assert!(f(100).is_none());
    }

    #[test]
    fn test_fibs() {
        test_fib("vec", fib_vec);
        test_fib("fancy", fib_vec_fancy);
        test_fib("array", fib_array);
        test_fib("registered", fib_registered);
        test_fib("fold", fib_fold);
        test_fib("lazy", fib_lazy);
        test_fib("closed", fib_closed);
        test_fib("lookup", fib_lookup);
    }
}
