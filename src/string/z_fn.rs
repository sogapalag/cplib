use std::cmp::min;

/// `z[i]` is longest common prefix(lcp) of `s` and `s[i..]`.
///
/// ### Time complexity
///
/// *O*(*n*)
//  Key is always record [l..r] the most-r matched z-box,
//  to max r-i as possible.
//  Then by [0..]<->[l..r] => [i-l..]<->[i..] => z[i-l] <-> z[i]
//  Thus z[i] >= min(r-i, z[i-l]).
pub fn z_fn<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 1;
    for i in 1..n {
        if i < r {
            z[i] = min(r - i, z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z[0] = n;
    z
}
