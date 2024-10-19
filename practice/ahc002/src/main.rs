#![allow(non_snake_case, dead_code)]

const N: usize = 50;

fn main() {
    get_time();
    let input = get_input();
    let mut state = State::new(&input);
    state.init_perm(&input);
    state.climb(&input);
    state.output();
}


struct DFS {
    // include start and end
    perm: Vec<Pos>,
    used_tile: Vec<bool>,
    score: isize,
    thre: f64,
    end: Option<Pos>,
    best_perm: Vec<Pos>,
    best_used_tile: Vec<bool>,
    best_score: isize,
    TL: f64,
}

use once_cell::sync::Lazy;
static SHUFFLED_DIRS: Lazy<Vec<Vec<usize>>> = Lazy::new(|| {
    (0..4).permutations(4).collect_vec()
});

impl DFS {
    fn dfs(&mut self, input: &Input, mt: &mut Mt) {
        let shuffled = &(&SHUFFLED_DIRS)[mt.gen_range(0..SHUFFLED_DIRS.len())];
        for d in shuffled {
            if get_time() > self.TL {
                break;
            }
            let q = self.perm[self.perm.len() - 1] + DIRS[*d].d;
            if !q.is_valid() {
                continue
            }
            self.perm.push(q);
            self.score += input.get_value(q);
            if self.end.map(|e| e == q).unwrap_or(false) {
                if self.thre < self.score as f64 {
                    self.best_perm = self.perm.clone();
                    self.best_used_tile = self.used_tile.clone();
                    self.best_score = self.score;
                    self.TL = 0.0;
                }
            }
            else if !self.used_tile[input.get_tile(q)] {
                self.used_tile[input.get_tile(q)] = true;
                if self.end.is_none() && self.best_score < self.score {
                    self.best_perm = self.perm.clone();
                    self.best_used_tile = self.used_tile.clone();
                    self.best_score = self.score;
                }
                self.dfs(input, mt);
                self.used_tile[input.get_tile(q)] = false;
            }
            self.score -= input.get_value(q);
            self.perm.pop();
        }
    }
}

#[derive(Clone)]
struct State {
    perm: Vec<Pos>,
    // [bool; M]
    used_tile: Vec<bool>, 
    score: isize,
}

impl State {
    fn new(input: &Input) -> State {
        State {
            perm: vec![],
            used_tile: vec![false; input.M],
            score: 0,
        }
    }

    fn init_dfs(&mut self, input: &Input, best_state: &mut State, TL: f64) {
        if best_state.score < self.score {
            *best_state = self.clone();
        }
        for d in 0..4 {
            if get_time() > TL {
                break
            }
            let q = self.perm[self.perm.len() - 1] + DIRS[d].d;
            if q.is_valid() && !self.used_tile[input.get_tile(q)] {
                self.used_tile[input.get_tile(q)] = true;
                self.perm.push(q);
                self.score += input.get_value(q);

                self.init_dfs(input, best_state, TL);

                self.score -= input.get_value(q);
                self.perm.pop();
                self.used_tile[input.get_tile(q)] = false;
            }
        }
    }

    fn init_perm(&mut self, input: &Input) {
        self.perm = vec![input.start];
        self.used_tile[input.get_tile(input.start)] = true;
        let mut best_state = self.clone();
        self.init_dfs(input, &mut best_state, 0.5);
        *self = best_state;
    }

    fn climb(&mut self, input: &Input) {
        let mut mt = Mt::new(768);
        //let mut frames = visualizer_shapes::Frames::new();
        //frames = frames.add_frame(self.answer_visualize(input));
        let mut iter = 0;

        let start = get_time();
        let T0 = 100.0f64;
        let T1 = 10.0f64;
        let mut T = T0;
        loop {
            let t = (get_time() - start) / (1.98 - start);
            if t > 1.0 {
                break;
            }
            //let before = get_time();
            iter += 1;
            if iter % 1000 == 0 {
                T = T0.powf(1.0 - t) * T1.powf(t);
            }
            if iter % 10000 == 0 {
                //frames = frames.add_frame(self.answer_visualize(input));
            }
            let len = mt.gen_range(2..20);
            let i = mt.gen_range(0..self.perm.len());
            let j = i + len;
            let mut score = - self.perm.get(j).map(|p| input.get_value(*p)).unwrap_or(0);
            for x in i + 1..j.min(self.perm.len()) {
                score -= input.get_value(self.perm[x]);
                self.used_tile[input.get_tile(self.perm[x])] = false;
            }

            //eprintln!("iteration: {}", iter);
            let mut dfs = DFS {
                perm: vec![self.perm[i]],
                used_tile: self.used_tile.clone(),
                score,
                end: self.perm.get(j).cloned(),
                thre: T * mt.gen_range(0.0f64..1.0).ln(),
                best_perm: vec![],
                best_used_tile: vec![],
                best_score: 0,
                TL: get_time() + 0.000_020,
            };
            //eprintln!("{:?} {:?}", T, dfs.thre);
            dfs.dfs(input, &mut mt);
            if dfs.best_perm.is_empty() {
                for x in i + 1..j.min(self.perm.len()) {
                    self.used_tile[input.get_tile(self.perm[x])] = true;
                }
                continue;
            }
            self.perm.splice(i..(j + 1).min(self.perm.len()), dfs.best_perm);
            self.used_tile = dfs.best_used_tile;
            //let after = get_time();
            //eprintln!("{} {} {}", after - before, before, after);
        }
        eprintln!("iteration: {}", iter);
        //frames = frames.add_frame(self.answer_visualize(input));
        //frames.encode_to_file("animate.vis");
    }

    fn output(&self) {
        for i in 1..self.perm.len() {
            for d in 0..4 {
                if self.perm[i - 1] + DIRS[d].d == self.perm[i] {
                    print!("{}", DIRS[d].c);
                }
            }
        }
        println!();
    }
    
    #[cfg(feature = "local")]
    fn answer_visualize(&self, input: &Input) -> visualizer_shapes::Frame {
        let mut frame = input.tile_visualize();
        for i in 0..N {
            for j in 0..N {
                if self.used_tile[input.get_tile(pos(i as isize, j as isize))] {
                    use visualizer_shapes::pos as ps;
                    frame = frame.add_element(
                        visualizer_shapes::Path::from_vertices(
                            vec![ps(i as f32, j as f32), ps(i as f32, j as f32 + 1.0), ps(i as f32 + 1.0, j as f32 + 1.0), ps(i as f32 + 1.0, j as f32)]
                        )
                        .stroke(visualizer_shapes::Color::new(0, 0, 0).alpha(0), 1.0)
                        .close(visualizer_shapes::Color::new(0, 255, 0).alpha(30))
                        .element()
                    );
                }
            }
        }
        for i in 0..self.perm.len() - 1 {
            frame = frame.add_element(
                visualizer_shapes::Path::new()
                .add_pos(visualizer_shapes::pos(self.perm[i].x as f32 + 0.5, self.perm[i].y as f32 + 0.5))
                .add_pos(visualizer_shapes::pos(self.perm[i + 1].x as f32 + 0.5, self.perm[i + 1].y as f32 + 0.5))
                .stroke(visualizer_shapes::Color::new(255, 0, 0), 2.0)
                .element()
            );
        }
        frame
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pos { x: isize, y: isize, }
impl Pos {
    fn norm1(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
    fn is_valid(&self) -> bool {
        0 <= self.x && self.x < N as isize && 0 <= self.y && self.y < N as isize
    }
}
fn pos(x: isize, y: isize) -> Pos { Pos { x, y, } }
impl Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        pos(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        pos(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul<isize> for Pos {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        pos(self.x * rhs, self.y * rhs)
    }
}

struct Dir { d: Pos, c: char, }

// 時計回り
const DIRS: [Dir; 4] = [
    Dir { d: Pos { x: 0, y: 1 }, c: 'R', },
    Dir { d: Pos { x: 1, y: 0 }, c: 'D', },
    Dir { d: Pos { x: 0, y: -1 }, c: 'L', },
    Dir { d: Pos { x: -1, y: 0 }, c: 'U', },
];

struct Input {
    start: Pos,
    M: usize,
    tile: [[usize; N]; N],
    value: [[usize; N]; N],
}

impl Input {
    fn get_tile(&self, p: Pos) -> usize {
        self.tile[p.x as usize][p.y as usize]
    }
    fn get_value(&self, p: Pos) -> isize {
        self.value[p.x as usize][p.y as usize] as isize
    }
    #[cfg(feature = "local")]
    fn tile_visualize(&self) -> visualizer_shapes::Frame {
        use visualizer_shapes::*;
        let mut frame = Frame::new(pos(-1.0, -1.0), pos(N as f32 + 1.0, N as f32 + 1.0));
        frame = frame
            .add_element(
                Path::from_vertices(vec![pos(0.0, 0.0), pos(0.0, N as f32), pos(N as f32, N as f32), pos(N as f32, 0.0), pos(0.0, 0.0)])
                    .stroke(Color::new(0, 0, 0), 1.0)
                    .element()
            );
        for i in 0..N {
            for j in 0..N {
                frame = frame.add_element(
                    Text::new(format!("{}", self.value[i][j]), 0.5, pos(i as f32 + 0.5, j as f32 + 0.5))
                    .element()
                );
            }
        }
        for i in 0..N - 1 {
            for j in 0..N {
                if self.tile[i][j] != self.tile[i + 1][j] {
                    frame = frame.add_element(
                        Path::from_vertices(vec![pos((i + 1) as f32, j as f32), pos((i + 1) as f32, (j + 1) as f32)])
                        .element()
                    );
                }
            }
        }

        for i in 0..N {
            for j in 0..N - 1 {
                if self.tile[i][j] != self.tile[i][j + 1] {
                    frame = frame.add_element(
                        Path::from_vertices(vec![pos(i as f32, (j + 1) as f32), pos((i + 1) as f32, (j + 1) as f32)])
                        .element()
                    );
                }
            }
        }
        frame
    }
}

fn get_input() -> Input {
    input! {
        sx: isize,
        sy: isize,
        ts: [[usize; N]; N],
        vs: [[usize; N]; N],
    }
    let mut tile = [[0; N]; N];
    let mut value = [[0; N]; N];
    let mut M = 0;
    for i in 0..N {
        for j in 0..N {
            tile[i][j] = ts[i][j];
            value[i][j] = vs[i][j];
            M = M.max(tile[i][j] + 1);
        }
    }
    Input {
        start: pos(sx, sy),
        M,
        tile,
        value,
    }
}

use std::ops::{ Add, Sub, Mul };

use itertools::Itertools;
use proconio::{input, input_interactive};
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
type Mt = Pcg64Mcg;

pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}
