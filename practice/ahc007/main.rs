fn main() {
    let Input { vs, es } = Input::from_stdin();
    let mut mt = Pcg64Mcg::new(768);
    let mut ans = vec![false; M];

    let mut cases = vec![vec![(!0, !0, !0, !0); M]; C];
    for c in 0..C {
        for i in 0..M {
            let (u, v) = es[i];
            let d = (vs[u].x - vs[v].x) * (vs[u].x - vs[v].x) + (vs[u].y - vs[v].y) * (vs[u].y - vs[v].y);
            let d = (d as f64).sqrt().round() as i64;
            cases[c][i] = (mt.gen_range(d..3 * d), i, u, v);
        }
        cases[c].sort();
    }
    let cases = cases;

    let mut uf = UnionFind::new(N);
    for i in 0..M {
        input_interactive!{ l: i64 }

        let mut sum = 0;
        if uf.root(es[i].0) != uf.root(es[i].1) {
            for case in cases.iter() {
                let mut uf2 = uf.clone();
                for (d, ei, u, v) in case.iter().cloned() {
                    if ei > i && uf2.unite(u, v).is_some() {
                        if uf2.root(es[i].0) == uf2.root(es[i].1) {
                            sum += d;
                            break;
                        }
                    }
                }
                if uf2.root(es[i].0) != uf2.root(es[i].1) {
                    sum = 1e18 as i64;
                }
            }
            if sum >= l * C as i64 {
                ans[i] = true;
            }
        }

        if ans[i] {
            uf.unite(es[i].0, es[i].1);
            println!("1");
        }
        else {
            println!("0");
        }
    }
}

const C: usize = 100;
const N: usize = 400;
const M: usize = 1995;

struct Pos {
    x: i64,
    y: i64,
}

struct Input {
    vs: Vec<Pos>,
    es: Vec<(usize, usize)>,
}

use proconio::input_interactive;
use rand_pcg::Pcg64Mcg;
use rand::prelude::*;

impl Input {
    fn from_stdin() -> Self {
        input_interactive! {
            xy: [(i64, i64); N],
            es: [(usize, usize); M],
        }
        Self {
            vs: xy.into_iter().map(|(x, y)| Pos { x, y }).collect(),
            es,
        }
    }
}

use std::cell::RefCell;

#[derive(Clone)]
pub struct UnionFind {
    par: RefCell<Vec<usize>>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: RefCell::new((0..n).collect()),
            sz: std::iter::repeat(1).take(n).collect(),
        }
    }

    pub fn root(&self, v: usize) -> usize {
        let p = self.par.borrow()[v];
        if p == v { v }
        else {
            let r = self.root(p);
            self.par.borrow_mut()[v] = r;
            r
        }
    }

    pub fn unite(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
        let a = self.root(a);
        let b = self.root(b);
        if a == b { None }
        else if self.sz[a] < self.sz[b] {
            self.par.borrow_mut()[a] = b;
            self.sz[b] += self.sz[a];
            Some((b, a))
        }
        else {
            self.par.borrow_mut()[b] = a;
            self.sz[a] += self.sz[b];
            Some((a, b))
        }
    }
}
