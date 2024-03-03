// https://qiita.com/rhoo/items/2f647e32f6ff2c6ee056

pub struct Input {}

#[derive(Clone, PartialEq)]
pub struct State {}
impl State {
    pub fn apply(&mut self, node: &Node) {
        todo!();
    }
    pub fn revert(&mut self, node: &Node) {
        todo!();
    }
}

#[derive(Clone)]
pub struct Cand {
    op: u8,
    parent: usize,
    eval_score: i64,
    hash: u64,
}

impl Cand {
    pub fn to_node(&self) -> Node {
        Node {
            child: !0,
            prev: !0,
            next: !0,
            op: self.op,
            parent: self.parent,
        }
    }
}

#[derive(Clone, Default)]
pub struct Node {
    op: u8,
    parent: usize,
    child: usize,
    prev: usize,
    next: usize,
}

const MAX_WIDTH: usize = 1000;
const TURN: usize = 100;

pub struct BeamSearch {
    state: State,
    leaf: Vec<usize>,
    next_leaf: Vec<usize>,
    nodes: Vec<Node>,
    cur_node: usize,
    free: Vec<usize>,
}

impl BeamSearch {
    pub fn new(state: State, node: Node) -> Self {
        const MAX_NODES: usize = MAX_WIDTH * 5;
        let mut nodes = vec![Node::default(); MAX_NODES];
        nodes[0] = node;
        let free = (1..MAX_NODES as usize).rev().collect();

        Self {
            state,
            leaf: vec![0],
            next_leaf: vec![],
            nodes,
            cur_node: 0,
            free,
        }
    }

    fn add_node(&mut self, cand: Cand) {
        let next = self.nodes[cand.parent as usize].child;
        let new = self.free.pop().expect("no free nodes") as usize;
        if next != !0 {
            self.nodes[next as usize].prev = new;
        }
        self.nodes[cand.parent as usize].child = new;
        self.next_leaf.push(new);
        self.nodes[new as usize] = Node { next, .. cand.to_node() };
    }

    fn del_node(&mut self, mut idx: usize) {
        loop {
            self.free.push(idx);
            let Node { prev, next, parent, .. } = self.nodes[idx as usize];
            assert_ne!(parent, !0, "cant delete all node");
            if prev == !0 && next == !0 {
                idx = parent;
                continue;
            }
            if prev != !0 {
                self.nodes[prev as usize].next = next;
            }
            else {
                self.nodes[parent as usize].child = next;
            }
            if next != !0 {
                self.nodes[next as usize].prev = prev;
            }
            break;
        }
    }

    fn enumerate_cands(&mut self, input: &Input, cands: &mut Vec<Cand>) {
        loop {
            let Node { next, child, .. } = self.nodes[self.cur_node];
            if next == !0 || child == !0 {
                break;
            }
            self.cur_node = child as usize;
            self.state.apply(&self.nodes[self.cur_node]);
        }

        let root = self.cur_node;

        loop {
            let child = self.nodes[self.cur_node].child;
            if child == !0 {
                self.append_cands(input, self.cur_node, cands);
                loop {
                    if self.cur_node == root {
                        return;
                    }
                    let node = &self.nodes[self.cur_node];
                    self.state.revert(&node);
                    if node.next != !0 {
                        self.cur_node = node.next as usize;
                        self.state.apply(&self.nodes[self.cur_node]);
                        break;
                    }
                    self.cur_node = node.parent as usize;
                }
            }
            else {
                self.cur_node = child as usize;
                self.state.apply(&self.nodes[self.cur_node]);
            }
        }
    }

    fn update<I: Iterator<Item=Cand>>(&mut self, cands: I) {
        self.next_leaf.clear();
        for cand in cands {
            self.add_node(cand);
        }

        for i in 0..self.leaf.len() {
            let n = self.leaf[i];
            if self.nodes[n as usize].child == !0 {
                self.del_node(n);
            }
        }

        std::mem::swap(&mut self.leaf, &mut self.next_leaf);
    }

    fn restore(&self, mut idx: usize) -> Vec<u8> {
        let mut ret = vec![];
        loop {
            let Node { op, parent, .. } = self.nodes[idx as usize];
            if op == !0 {
                break;
            }
            ret.push(op);
            idx = parent;
        }

        ret.reverse();
        ret
    }

    fn append_cands(&self, input: &Input, idx: usize, cands: &mut Vec<Cand>) {
        let node = &self.nodes[idx];
        assert_eq!(node.child, !0);
        todo!();
    }
}
