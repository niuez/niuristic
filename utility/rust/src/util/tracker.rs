use rand::Rng;

pub trait Tracker {
    type T;
    fn empty() -> Self;
    fn clear(&mut self);
    fn push(&mut self, t: Self::T);
    fn pick(&self) -> Option<&Self::T>;
    fn random_pick<R: Rng>(&self, rng: &mut R) -> Option<&Self::T>;
}

pub struct OneTracker<T>(Option<T>);

impl<T> Tracker for OneTracker<T> {
    type T = T;
    fn empty() -> Self {
        Self(None)
    }
    fn clear(&mut self) {
        self.0 = None
    }
    fn push(&mut self, t: T) {
        self.0 = Some(t);
    }
    fn pick(&self) -> Option<&T> {
        self.0.as_ref()
    }
    fn random_pick<R: Rng>(&self, _: &mut R) -> Option<&T> {
        self.0.as_ref()
    }
}

pub struct MultiTracker<T>(Vec<T>);

impl<T> Tracker for MultiTracker<T> {
    type T = T;
    fn empty() -> Self {
        Self(Vec::new())
    }
    fn clear(&mut self) {
        self.0.clear()
    }
    fn push(&mut self, t: T) {
        self.0.push(t);
    }
    fn pick(&self) -> Option<&T> {
        self.0.get(0)
    }
    fn random_pick<R: Rng>(&self, r: &mut R) -> Option<&T> {
        if self.0.len() > 0 {
            self.0.get(r.gen_range(0..self.0.len()))
        }
        else {
            None
        }
    }
}

pub struct Tracking<T, Tr: Tracker<T = (usize, T)>> {
    trs: Vec<Tr>,
}

impl<T, Tr: Tracker<T = (usize, T)>> Tracking<T, Tr> {
    pub fn new(n: usize) -> Self {
        Self {
            trs: (0..n).map(|_| Tracker::empty()).collect(),
        }
    }

    pub fn clear_backtrack(&mut self, v: usize) {
        self.trs[v].clear();
    }

    pub fn add_edge(&mut self, from: usize, to: usize, t: T) {
        self.trs[to].push((from, t));
    }

    pub fn path(&mut self, from: usize, to: usize) -> Option<Vec<(usize, &T)>> {
        let mut ans = vec![];
        let mut v = to;
        while v != from {
            let Some(&(next, ref t)) = self.trs[v].pick() else { return None };
            ans.push((next, t));
            v = next;
        }

        ans.reverse();
        Some(ans)
    }

    pub fn random_path<R: Rng>(&mut self, from: usize, to: usize, rng: &mut R) -> Option<Vec<(usize, &T)>> {
        let mut ans = vec![];
        let mut v = to;
        while v != from {
            let Some(&(next, ref t)) = self.trs[v].random_pick(rng) else { return None };
            ans.push((next, t));
            v = next;
        }

        ans.reverse();
        Some(ans)
    }
}

#[test]
fn multi_tracker() {
    let mut trs = Tracking::<char, MultiTracker<_>>::new(9);
    for i in 0..3 {
        for j in 0..3 {
            if i + 1 < 3 {
                trs.add_edge(i * 3 + j, (i + 1) * 3 + j, 'D');
            }
            if j + 1 < 3 {
                trs.add_edge(i * 3 + j, i * 3 + j + 1, 'R');
            }
        }
    }

    let mut mt = rand_pcg::Pcg64Mcg::new(768);
    for _ in 0..5 {
        println!("{}", trs.random_path(0, 8, &mut mt).unwrap().into_iter().map(|(_, c)| *c).collect::<String>());
    }
}
