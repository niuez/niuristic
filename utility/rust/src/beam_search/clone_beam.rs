#[allow(unused)]
pub mod clonebeam {
    use crate::util::nop_hash::{ NopHashSet, NopHashMap };
    use std::cmp::Ordering;

    pub trait BeamState: Clone + Default {
        type Action: Default + Clone + std::fmt::Debug;
        type Candidates: IntoIterator<Item=(Self, Self::Action)>;
        type Score: Default + Copy + Ord;
        fn next_states(&self) -> Self::Candidates;
        fn score(&self) -> Self::Score;
        fn hash(&self) -> u64;
    }

    pub struct BeamSearch<State: BeamState> {
        pub cur: Vec<(State, usize)>,
        pub sz: usize,
        nxt: Vec<(State, usize)>,
        acts: Vec<(State::Action, usize)>,
        act_i: usize,
    }

    impl<State: BeamState> BeamSearch<State> {
        pub fn new(width: usize, max_states: usize, max_acts: usize, root_state: State) -> Self {
            let mut cur = vec![(State::default(), !0); width];
            let nxt = vec![(State::default(), !0); max_states];
            cur[0].0 = root_state;
            Self {
                cur,
                sz: 1,
                nxt,
                acts: vec![Default::default(); max_acts],
                act_i: 0,
            }
        }

        pub fn extend(&mut self) {
            let mut nxt_sz = 0;
            for i in 0..self.sz {
                for (next_state, next_act) in self.cur[i].0.next_states() {
                    self.nxt[nxt_sz].0 = next_state;
                    self.nxt[nxt_sz].1 = self.act_i;
                    self.acts[self.act_i] = (next_act, self.cur[i].1);
                    self.act_i += 1;
                }
            }
            self.nxt.sort_by_key(|s| s.0.score());
            self.sz = 0;
            let mut st = NopHashSet::default();
            for nx in self.nxt.iter_mut() {
                if st.insert(nx.0.hash()) {
                    std::mem::swap(&mut self.cur[self.sz], nx);
                    self.sz += 1;
                }
            }
        }

        pub fn build_actions(&self, mut i: usize) -> Vec<State::Action> {
            let mut res = vec![];
            while i != !0 {
                res.push(self.acts[i].0.clone());
                i = self.acts[i].1;
            }
            res.reverse();
            res
        }
    }
}
