#include <vector>
#include <cassert>
#include <iostream>
#include <numeric>

template<class T>
struct Matrix: public std::vector<std::vector<T>> {
  int H;
  int W;
  Matrix() {}
  Matrix(int H, int W, T t = T())
    : H(H),
      W(W),
      std::vector<std::vector<T>>(H, std::vector<T>(W, t)) {}
  Matrix(std::vector<std::vector<T>> vec)
      : std::vector<std::vector<T>>(vec) {
        this->H = this->size();
        this->W = this->empty() ? 0 : (*this)[0].size();
      }
};

using D = double;

std::vector<D> linsolve(Matrix<D> A, std::vector<D> b) {
  // http://www.ced.is.utsunomiya-u.ac.jp/lecture/2011/prog/p2/kadai3/no3/lu.pdf
  // https://www.cspp.cc.u-tokyo.ac.jp/hanawa/class/spc2016s/sp20160614-2.pdf
  // LU Decomposition: right-looking algorithm
  // Ax = LUx = b
  assert(A.H == A.W);
  int n = A.H;
  for(int k=0; k<n; k++) {
    D dtmp = 1.0 / A[k][k];
    for (int i=k+1; i<n; i++) {
      A[i][k] = A[i][k]*dtmp;
    }
    for (int j=k+1; j<n; j++) {
      D dakj = A[k][j];
      for (int i=k+1; i<n; i++) {
        A[i][j] = A[i][j] - A[i][k]*dakj;
      }
    }
  }
  // Ly = b
  for(int i = 0; i < n; i++) {
    D s = b[i];
    for(int j = 0; j < i; j++) {
      s -= A[i][j] * b[j];
    }
    b[i] = s; // L[i][i] = 1
  }
  // Ux = y
  for(int i = n; i --> 0;) {
    D s = b[i];
    for(int j = n; j --> i + 1;) {
      s -= A[i][j] * b[j];
    }
    b[i] = s / A[i][i];
  }
  return b;
}

struct ModifiedCholeskyDecomposition {
  int n;
  Matrix<D> A;
  std::vector<D> d;
  ModifiedCholeskyDecomposition(Matrix<D> a): A(std::move(a)) {
    n = A.H;
    d.resize(n);

    A[0][0] = A[0][0];
    d[0] = 1.0 / A[0][0];

    for(int i = 1; i < n; ++i){
      for(int j = 0; j <= i; ++j){
        double lld = A[i][j];
        for(int k = 0; k < j; ++k){
          lld -= A[i][k]*A[j][k]*d[k];
        }
        A[i][j] = lld;
        A[j][i] = lld;
      }
      d[i] = 1.0/ A[i][i];
    }
  }

  std::vector<D> solve(std::vector<D> b) const {
    // LDy = b
    for(int i = 0; i < n; i++) {
      D s = b[i];
      for(int j = 0; j < i; j++) {
        s -= A[i][j] * d[j] * b[j];
      }
      b[i] = s; // L[i][i] = 1
    }
    // Ux = y
    for(int i = n; i --> 0;) {
      D s = b[i];
      for(int j = n; j --> i + 1;) {
        s -= A[i][j] * b[j];
      }
      b[i] = s / A[i][i];
    }
    return b;
  }
};

std::vector<D> symmetric_linsolve(Matrix<D> A, std::vector<D> b) {
  // https://www.slis.tsukuba.ac.jp/~fujisawa.makoto.fu/cgi-bin/wiki/?%A5%B3%A5%EC%A5%B9%A5%AD%A1%BC%CA%AC%B2%F2
  // Modified Cholesky Decomposition
  // Ax = LDL^T x = b
  assert(A.H == A.W);
  int n = A.H;

  std::vector<D> d(n);
  A[0][0] = A[0][0];
  d[0] = 1.0 / A[0][0];

  for(int i = 1; i < n; ++i){
    for(int j = 0; j <= i; ++j){
      double lld = A[i][j];
      for(int k = 0; k < j; ++k){
        lld -= A[i][k]*A[j][k]*d[k];
      }
      A[i][j] = lld;
      A[j][i] = lld;
    }
    d[i] = 1.0/ A[i][i];
  }
  // LDy = b
  for(int i = 0; i < n; i++) {
    D s = b[i];
    for(int j = 0; j < i; j++) {
      s -= A[i][j] * d[j] * b[j];
    }
    b[i] = s; // L[i][i] = 1
  }
  // Ux = y
  for(int i = n; i --> 0;) {
    D s = b[i];
    for(int j = n; j --> i + 1;) {
      s -= A[i][j] * b[j];
    }
    b[i] = s / A[i][i];
  }
  return b;
}

struct Input {
  int N;
  int W;
  int K;
  int C;

  std::vector<std::pair<int, int>> waters;
  std::vector<std::pair<int, int>> houses;
  inline int index(int i, int j) const {
    return i * N + j;
  }
};

struct InlineJudge {
  int N;
  std::vector<std::vector<int>> field;
  InlineJudge(int n, std::istream& is): N(n), field(n, std::vector<int>(n)) {
    for(int i = 0; i < n; i++) {
      for(int j = 0; j < n; j++) {
        is >> field[i][j];
      }
    }
  }
};

template<class Judge>
std::tuple<Input, Judge> get_input(std::istream& is) {
  Input in;
  is >> in.N >> in.W >> in.K >> in.C;
  
  Judge judge(in.N, is);

  in.waters.resize(in.W);
  for(int i = 0; i < in.W; i++) {
    is >> in.waters[i].first >> in.waters[i].second;
  }
  in.houses.resize(in.K);
  for(int i = 0; i < in.K; i++) {
    is >> in.houses[i].first >> in.houses[i].second;
  }
  return std::make_tuple(std::move(in), std::move(judge));
}

#include <cmath>

struct RBFKernel {
  D alpha, beta, gamma;
  RBFKernel(D a, D b, D g): alpha(a), beta(b), gamma(g) {}
  D operator()(const int xi, const std::vector<D>& x, const int yi, const std::vector<D>& y) {
    double d = 0;
    for(int i = 0; i < x.size(); i++) {
      d += (x[i] - y[i]) * (x[i] - y[i]);
    }
    return alpha * std::exp(-d/beta) + (xi == yi ? gamma : 0);
  }
};

int main() {
  auto [in, judge] = get_input<InlineJudge>(std::cin);
  RBFKernel rbf(0.1, 0.01, 0);
  const int window = 20;
  const int S = (in.N / window) * (in.N / window);
  Matrix<D> K(S, S);
  std::vector<std::vector<D>> x(S);
  std::vector<D> y(S);
  int iter = 0;
  for(int i = 0; i < in.N / window; i++) {
    for(int j = 0; j < in.N / window; j++) {
      x[iter] = { D(i * window) / D(in.N), D(j * window) / D(in.N) };
      y[iter] = std::sqrt(D(judge.field[i * window][j * window]));
      iter++;
    }
  }
  for(int i = 0; i < S; i++) {
    for(int j = 0; j <= i; j++) {
      K[i][j] = K[j][i] = rbf(i, x[i], j, x[j]);
    }
  }
  /*
  for(int i = 0; i < S; i++) {
    for(int j = 0; j < S; j++) {
      std::cerr << K[i][j] << " \n"[j + 1 == S];
    }
  }
  */
  ModifiedCholeskyDecomposition mcd(K);
  for(int i = 0; i < in.N; i++) {
    for(int j = 0; j < in.N; j++) {
      std::vector<D> xd = { D(i) / D(in.N), D(j) / D(in.N) };
      std::vector<D> ka(S);
      for(int k = 0; k < S; k++) {
        ka[k] = rbf(S, xd, k, x[k]);
      }
      D kb = rbf(S, xd, S, xd);
      auto ktk = mcd.solve(ka);
      D mu = 0;
      D sigma = kb;
      for(int k = 0; k < S; k++) {
        //std::cerr << k << " " << ktk[k] << std::endl;
        mu += ktk[k] * y[k];
        sigma -= ktk[k] * ka[k];
      }
      std::cout << std::clamp(int(mu * mu), 10, 5000) << " ";
      std::cout << std::flush;
    }
    std::cout << std::endl;
  }
}
