use std::ops::Range;

/// `res[i]`: for `s[0..=i]`, len of longest non-trivial prefix also be suffix.
//  Property 1: res[i+1] <= res[i]+1.
//  Since any prefix of i+1, remove its last letter, result prefix of i.
//  Property 2: if c <. b and b <. a, then c <.. a
//  Where <. means the longest non-trivial. <.. as second longest.
pub fn prefix_fn<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut res = vec![0; n];
    for i in 1..n {
        let mut j = res[i - 1];
        while j > 0 && s[i] != s[j] {
            j = res[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        res[i] = j;
    }
    res
}

/// Matched `[0..i]`, with next char => len of matched now?
pub fn prefix_automaton(s: &[u8], alphabet: Range<u8>) -> Vec<Vec<usize>> {
    let mut s = s.iter().map(|&x| Some(x)).collect::<Vec<_>>();
    s.push(None);
    let pi = prefix_fn(&s);
    let Range { start: l, end: r } = alphabet;
    let n = s.len();
    let mut trans = vec![vec![0; (r - l) as usize]; n];
    for i in 0..n {
        for c in l..r {
            trans[i][(c - l) as usize] = if i > 0 && Some(c) != s[i] {
                trans[pi[i - 1]][(c - l) as usize]
            } else {
                i + if Some(c) == s[i] { 1 } else { 0 }
            }
        }
    }
    trans
}
