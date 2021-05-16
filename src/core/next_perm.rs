pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let last_inc = match a.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            // finished entire sequence of perm.
            a.reverse();
            return false;
        }
    };
    // last_inc < n-1 ensure Some.
    let last_greater = a.iter().rposition(|x| a[last_inc] < *x).unwrap();
    a.swap(last_inc, last_greater);
    a[last_inc + 1..].reverse();
    true
}
