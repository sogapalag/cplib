pub fn accum<I, B, F>(iter: I, init: B, f: F) -> Accum<I, B, F> {
    Accum::new(iter, init, f)
}
#[derive(Clone)]
pub struct Accum<I, B, F> {
    iter: I,
    f: F,
    state: Option<B>,
}
impl<I, B, F> Accum<I, B, F> {
    fn new(iter: I, init: B, f: F) -> Accum<I, B, F> {
        Accum {
            iter,
            state: Some(init),
            f,
        }
    }
}
impl<I, B: Clone, F> Iterator for Accum<I, B, F>
where
    I: Iterator,
    F: FnMut(B, I::Item) -> B,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        let res = self.state.clone();
        match self.state.as_mut() {
            Some(v) => match self.iter.next() {
                Some(a) => *v = (self.f)((*v).clone(), a),
                None => self.state = None,
            },
            None => {}
        }
        res
    }
}
