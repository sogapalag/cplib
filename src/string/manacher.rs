use std::cmp::min;

/// `a[i]` is palindrome length with `i`th center of `2*n-1` centers.
//  Key: record most-r box [l..=r]
pub fn manacher<T: PartialEq + Copy>(text: &[T]) -> Vec<usize> {
    // a#b#...#c
    let n = text.len();
    let mut s = Vec::with_capacity(2 * n - 1);
    s.push(Some(text[0]));
    for i in 1..n {
        s.push(None);
        s.push(Some(text[i]));
    }
    let n = 2 * n - 1;
    let mut a = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    for i in 0..n {
        let mut k = 1;
        if i < r {
            k = min(a[l + r - i], r - i + 1);
        }
        while k <= i && i + k < n && s[i - k] == s[i + k] {
            k += 1;
        }
        k -= 1;
        if i + k > r {
            l = i - k;
            r = i + k;
        }
        a[i] = k;
    }
    // radius -> len
    for i in 0..n {
        if i % 2 == 0 {
            a[i] = a[i] / 2 * 2 + 1;
        } else {
            a[i] = (a[i] + 1) / 2 * 2;
        }
    }
    a
}
