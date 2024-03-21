use nalgebra::{ DMatrix, DVector, LU, Dyn };

#[derive(Debug, Clone)]
pub struct RFBKernel {
    alpha: f64,
    beta: f64,
    gamma: f64,
}

impl RFBKernel {
    pub fn new(alpha: f64, beta: f64, gamma: f64) -> Self {
        Self { alpha, beta, gamma, }
    }
    pub fn kernel(&self, ai: usize, a: &DVector<f64>, bi: usize, b: &DVector<f64>) -> f64 {
        let diff = a - b;
        let d = diff.component_mul(&diff).sum();
        let v = self.alpha * (-d / self.beta).exp();
        if ai == bi {
            v + self.gamma
        }
        else {
            v
        }
    }
}

#[derive(Debug, Clone)]
pub struct GaussianProcessBuilder {
    xs: Vec<DVector<f64>>,
    ys: Vec<f64>,
    kernel: RFBKernel,
}


impl GaussianProcessBuilder {
    pub fn new(a: f64, b: f64, g: f64) -> Self {
        Self {
            xs: vec![],
            ys: vec![],
            kernel: RFBKernel::new(a, b, g),
        }
    }

    pub fn add_data(&mut self, x: DVector<f64>, y: f64) {
        self.xs.push(x);
        self.ys.push(y)
    }

    pub fn clear(&mut self) {
        self.xs.clear();
        self.ys.clear();
    }

    fn kernel_matrix(&self) -> DMatrix<f64> {
        let k = (0..self.xs.len()).map(|i| {
            (0..self.xs.len()).map(move |j| {
                self.kernel.kernel(i, &self.xs[i], j, &self.xs[j])
            })
        }).flatten();
        let k = DMatrix::from_iterator(self.xs.len(), self.xs.len(), k);
        k
    }

    pub fn build(&self) -> GaussianProccessRegression<'_> {
        let mut ys = DVector::from_vec(self.ys.clone());
        let ymean = ys.mean();
        ys.add_scalar_mut(-ymean);
        let k = self.kernel_matrix();
        let lu = k.lu();
        GaussianProccessRegression {
            xs: &self.xs,
            ys,
            ymean,
            k_inv: lu,
            kernel: &self.kernel,
        }
    }
}

#[derive(Debug)]
pub struct GaussianProccessRegression<'a> {
    xs: &'a Vec<DVector<f64>>,
    ys: DVector<f64>,
    ymean: f64,
    k_inv: LU<f64, Dyn, Dyn>,
    kernel: &'a RFBKernel,
}

impl<'a> GaussianProccessRegression<'a> {
    pub fn regression(&self, x_pred: &DVector<f64>) -> (f64, f64) {
        let s = self.xs.len();
        let ka = self.xs.iter().enumerate()
            .map(|(i, x)| self.kernel.kernel(i, x, s, x_pred))
            .collect::<Vec<_>>();
        let ka = DVector::from_vec(ka);
        let kb = self.kernel.kernel(s, x_pred, s, x_pred);
        let ktk = self.k_inv.solve(&ka).unwrap();
        let mu = ktk.dot(&self.ys);
        let sigma = kb - ktk.dot(&ka);
        (mu + self.ymean, sigma)
    }
}
