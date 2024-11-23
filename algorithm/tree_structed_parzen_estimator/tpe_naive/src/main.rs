use rand::Rng;
use rand_distr::Distribution;

struct ParzenEstimator {
    low: f64,
    high: f64,
    x: Vec<f64>,
    stddev: f64,
}

impl ParzenEstimator {
    fn init<I: IntoIterator<Item=f64>>(x: I, stddev: f64, low: f64, high: f64) -> Self {
        ParzenEstimator {
            low,
            high,
            x: x.into_iter().collect(),
            stddev,
        }
    }
    // [(x, P(x)]
    fn samples<R: Rng>(&self, n: usize, rnd: &mut R) -> Vec<(f64, f64)> {
        (0..n).map(|_| {
            let idx = rnd.gen_range(0..self.x.len());
            let dist = rand_distr::Normal::<f64>::new(self.x[idx], self.stddev).unwrap();
            let s = dist.sample(rnd).max(self.low).min(self.high);
            (s, self.pdf(s))
        }).collect()
    }

    fn pdf(&self, s: f64) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.x.len() {
            let d = (s - self.x[i]) / self.stddev;
            let p = (-0.5 * d * d).exp() / (2.0 * std::f64::consts::PI).sqrt() / self.stddev;
            sum += p / self.x.len() as f64;
        }
        sum
    }
}

struct Trial {
    x: Vec<f64>,
    y: f64,
}

struct TPENaive {
    dim: usize,
    low: Vec<f64>,
    high: Vec<f64>,
    data: Vec<Trial>,
    bandwidth: f64,
    gamma: f64,
    mt: rand_pcg::Pcg64Mcg, 
}

impl TPENaive {
    pub fn init(low: Vec<f64>, high: Vec<f64>) -> Self {
        Self {
            dim: low.len(),
            low,
            high,
            data: Vec::new(),
            bandwidth: 0.2,
            gamma: 0.2,
            mt: rand_pcg::Pcg64Mcg::new(768),
        }
    }

    pub fn add_trial(&mut self, x: Vec<f64>, y: f64) {
        self.data.push(Trial { x, y });
    }

    pub fn suggest_by_tpe(&mut self) -> Vec<f64> {
        let best_n = (self.gamma * self.data.len() as f64).floor() as usize;
        self.data.select_nth_unstable_by(best_n, |a, b| a.y.partial_cmp(&b.y).unwrap());

        let mut suggest = vec![0f64; self.dim];
        for d in 0..self.dim {
            let l_dist = ParzenEstimator::init(self.data[0..best_n].iter().map(|t| t.x[d]), self.bandwidth, self.low[d], self.high[d]);
            let g_dist = ParzenEstimator::init(self.data[best_n..self.data.len()].iter().map(|t| t.x[d]), self.bandwidth, self.low[d], self.high[d]);

            // なぜか偏る
            // achley(意図的な多峰性)に弱いのかもしれない
            //let samples = l_dist.samples(100, &mut self.mt);

            let samples = (0..100).map(|_| {
                let s = self.mt.gen_range(self.low[d]..self.high[d]);
                (s, l_dist.pdf(s))
            }).collect::<Vec<_>>();
            let g_p = samples.iter().map(|t| g_dist.pdf(t.0)).collect::<Vec<_>>();

            let mut max_lig = std::f64::MIN;
            for i in 0..samples.len() {
                let lig = samples[i].1 / (g_p[i] + 1e-12);
                if lig > max_lig {
                    max_lig = lig;
                    suggest[d] = samples[i].0;
                }
            }
        }
        suggest
    }

    pub fn suggest(&mut self, uniform_rand: bool) -> Vec<f64> {
        if uniform_rand {
            (0..self.dim).map(|i| self.mt.gen_range(self.low[i]..self.high[i])).collect::<Vec<_>>()
        }
        else {
            self.suggest_by_tpe()
        }
    }
}

pub fn ackley(x: f64, y: f64) -> f64 {
    // ackley function
    0.0
        + 20.0
        - 20.0 * (
            -0.2 * (0.5 * (x * x + y * y)).sqrt()
          ).exp()
        + std::f64::consts::E
        - (0.5 * (
                  (2.0 * std::f64::consts::PI * x).cos()
                + (2.0 * std::f64::consts::PI * y).cos()
                )
          ).exp()
}

pub fn rosenbrock(x: f64, y: f64) -> f64 {
    // rosenbrock function
    100.0 * (y - x * x).powi(2) + (1.0 - x).powi(2)
}

pub fn booth_function(x: f64, y: f64) -> f64 {
    // booth function
    (x + 2.0 * y - 7.0).powi(2) + (2.0 * x + y - 5.0).powi(2)
}

fn main() {
    let mut tpe = TPENaive::init(vec![-10.0, -10.0], vec![10.0, 10.0]);
    let mut best = std::f64::MAX;
    for t in 0..1000 {
        let x = tpe.suggest(t < 40);
        let y = booth_function(x[0], x[1]);
        best = best.min(y);
        eprintln!("{}\t{}\t{}\t{}\t{}", t, x[0], x[1], y, best);
        tpe.add_trial(x, y);
    }
}
