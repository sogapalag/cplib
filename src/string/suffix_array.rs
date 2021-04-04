use crate::algebra::Min;
use crate::ds::SparseTable;

type P = Vec<usize>;

/// A efficient data structure to support view of suffix trie.
///
/// For suffix lcp and substring task.
/// ### Time complexity
///
/// Build, *O*(*n*log *n*).
/// Query lcp, *O*(1)
pub struct SuffixArray {
    /// `sa`: indexes sorted by suffix string.
    pub sa: P,
    /// `rk[i]`: rank of `s[i..]`, note `rk * sa = I`.
    pub rk: P,
    /// `lcp[i] = lcp(rk=i, rk=i+1)`.
    pub lcp: P,
    st: SparseTable<usize, Min>,
}

impl SuffixArray {
    pub fn new<T>(s: &[T]) -> Self
    where
        T: Into<usize> + Copy + PartialEq,
    {
        let (sa, rk, lcp) = create(s);
        let st = SparseTable::new(&lcp);
        Self { sa, rk, lcp, st }
    }
    /// Get lcp of `s[i..]` and `s[j..]`.
    pub fn lcp(&self, i: usize, j: usize) -> usize {
        self.lcp_of_rank(self.rk[i], self.rk[j])
    }
    /// Get lcp of ranked `x` and `y` suffix.
    pub fn lcp_of_rank(&self, x: usize, y: usize) -> usize {
        let (x, y) = if x < y { (x, y) } else { (y, x) };
        self.st.query(x..y + 1)
    }
}

// Radix sort on (x,y), since y is sorted now, just need to sort by x.
// Thus we get sa of len=2k
fn counting_sort(x: &P, y: &P, sa: &mut P, m: usize) {
    let mut cnt = vec![0; m];
    for &i in y {
        cnt[x[i]] += 1;
    }
    for p in 1..m {
        cnt[p] += cnt[p - 1];
    }
    for &i in y.iter().rev() {
        cnt[x[i]] -= 1;
        sa[cnt[x[i]]] = i;
    }
}
// (x,y)_i <-> ([i..i+k], [i+k..i+2k])'s rank
// y: [n] sorted by rank, y=i <-> [i+k..i+2k]
// x: rank array, x[i] = rank of [i..i+k]
// m: size of equivalent class, i.e. same string under current length.
// p: equivalent class counter.
// sa: suffix array, [n] sorted by s[i..i+]
// rk: rank of s[i..]
// lcp: lcp[i] = lcp(rk=i, rk=i+1)
fn create<T>(s: &[T]) -> (P, P, P)
where
    T: Into<usize> + Copy + PartialEq,
{
    let n = s.len();
    let mut x: P = vec![0; n + 1];
    for i in 0..n {
        x[i] = s[i].into() + 1;
    }
    let mut y: P = (0..=n).collect();
    let mut sa: P = vec![0; n + 1];
    let n = n + 1;
    let mut m = 257;

    // build suffix array
    counting_sort(&x, &y, &mut sa, m);
    let mut k = 1;
    while k < n {
        // sort y first
        let mut p = 0;
        for i in n - k..n {
            y[p] = i;
            p += 1;
        }
        for &i in &sa {
            if i >= k {
                y[p] = i - k;
                p += 1;
            }
        }
        // then sort x
        counting_sort(&x, &y, &mut sa, m);
        // prep x <-> [i..i+2k]
        std::mem::swap(&mut x, &mut y);
        assert_eq!(sa[0], n - 1);
        x[sa[0]] = 0;
        let mut p = 0;
        for j in 1..n {
            // equivalent?
            if y[sa[j - 1]] < y[sa[j]] || y[sa[j - 1] + k] < y[sa[j] + k] {
                p += 1;
            }
            x[sa[j]] = p;
        }
        m = p + 1;
        if m >= n {
            break;
        }
        k *= 2;
    }
    // back to 0-based
    let n = n - 1;
    for i in 0..n {
        sa[i] = sa[i + 1];
    }
    sa.pop();
    // rk * sa = I
    let mut rk = x;
    rk.pop();
    for i in 0..n {
        rk[sa[i]] = i;
    }
    // Kasai's
    // https://web.stanford.edu/class/archive/cs/cs166/cs166.1206/lectures/03/Small03.pdf
    // lcp(i,j)=h  =>  lcp(i+1,j+1)>=h-1  =>  lcp(i+1, k) >= h-1
    let mut lcp = y;
    lcp.pop();
    let mut h = 0_usize;
    for i in 0..n {
        if rk[i] + 1 < n {
            h = h.saturating_sub(1);
            let j = sa[rk[i] + 1];
            while i + h < n && j + h < n && s[i + h] == s[j + h] {
                h += 1;
            }
            lcp[rk[i]] = h;
        }
    }
    lcp[n - 1] = 0;
    (sa, rk, lcp)
}
