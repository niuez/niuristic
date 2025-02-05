use nalgebra::{ Cholesky, DMatrix, DVector, };

#[derive(Debug, Clone)]
pub struct KalmanFilter {
    m: DVector<f64>,
    s: DMatrix<f64>,
}

impl KalmanFilter {
    pub fn new(m: DVector<f64>, s: DMatrix<f64>) -> Self {
        Self {
            m,
            s,
        }
    }

    pub fn step(&mut self, c: DMatrix<f64>, y: DVector<f64>, r: DMatrix<f64>) {
        let cs = c.clone() * self.s.clone();
        let cscr = cs.clone() * c.transpose() + r;
        let scsr_inv = Cholesky::new(cscr).unwrap();
        let k = scsr_inv.solve(&cs).transpose();
        self.s -= k.clone() * cs;
        self.m += k * (y - c * self.m.clone());
    }
}
