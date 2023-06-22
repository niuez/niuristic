#pragma once
#include "../matrix/matrix.hpp"
#include "../matrix/cholesky_decomposition.hpp"
#include <cmath>


template<class D>
struct RBFKernel {
  static constexpr int DDim = 3;
  D alpha, beta, gamma;
  RBFKernel(D a, D b, D g): alpha(a), beta(b), gamma(g) {}
  D operator()(const int xi, const std::vector<D>& x, const int yi, const std::vector<D>& y) const {
    double d = 0;
    for(int i = 0; i < x.size(); i++) {
      d += (x[i] - y[i]) * (x[i] - y[i]);
    }
    return alpha * std::exp(-d/beta) + (xi == yi ? gamma : 0);
  }
  std::vector<D> diff(const int xi, const std::vector<D>& x, const int yi, const std::vector<D>& y) const {
    double d = 0;
    for(int i = 0; i < x.size(); i++) {
      d += (x[i] - y[i]) * (x[i] - y[i]);
    }
    double k = alpha * std::exp(-d/beta);
    double g = (xi == yi ? gamma : 0);
    return { k, k / beta * d, g };
  }

  void scg(const D eta, const std::vector<D>& delta) {
    alpha = std::exp(std::log(alpha) + eta * delta[0]);
    beta  = std::exp(std::log(beta ) + eta * delta[1]);
    gamma = std::exp(std::log(gamma) + eta * delta[2]);
  }
};

struct GaussianProcessRegression {
  using D = double;
  using Kernel = RBFKernel<D>;
  int S = 0;
  std::vector<std::vector<D>> X;
  std::vector<D> Y;
  D Ymu;
  std::vector<D> normY;

  Matrix<D> K;
  Kernel kernel;
  ModifiedCholeskyDecomposition<D> linsol;
  GaussianProcessRegression(Kernel kernel): kernel(std::move(kernel)) {}
  void reserve(int s) {
    X.reserve(s);
    Y.reserve(s);
  }
  void add(std::vector<D> x, D y) {
    S++;
    X.push_back(std::move(x));
    Y.push_back(std::move(y));
  }
  void build() {
    K = Matrix<D>(S, S);
    Ymu = 0;
    for(int i = 0; i < S; i++) {
      for(int j = 0; j <= i; j++) {
        K[i][j] = K[j][i] = kernel(i, X[i], j, X[j]);
      }
      Ymu += Y[i];
    }
    Ymu /= D(S);
    normY.resize(S);
    for(int i = 0; i < S; i++) {
      normY[i] = Y[i] - Ymu;
    }
    linsol.init(K);
  }
  inline std::pair<D, D> reg(const std::vector<D>& xd) const {
    std::vector<D> ka(S);
    for(int k = 0; k < S; k++) {
      ka[k] = kernel(S, xd, k, X[k]);
    }
    D kb = kernel(S, xd, S, xd);
    auto ktk = linsol.solve(ka);
    D mu = 0;
    D sigma = kb;
    for(int k = 0; k < S; k++) {
      mu += ktk[k] * normY[k];
      sigma -= ktk[k] * ka[k];
    }
    return { mu + Ymu, sigma };
  }
  void scg(const D eta) {
    std::vector<Matrix<D>> Kd(kernel.DDim, Matrix<D>(S, S));
    for(int i = 0; i < S; i++) {
      for(int j = 0; j <= i; j++) {
        auto di = kernel.diff(i, X[i], j, X[j]);
        for(int k = 0; k < kernel.DDim; k++) {
          Kd[k][i][j] = Kd[k][j][i] = di[k];
        }
      }
    }
    std::vector<D> delta(kernel.DDim);
    std::vector<D> kty = linsol.solve(normY);
    for(int k = 0; k < kernel.DDim; k++) {
      for(int i = 0; i < S; i++) {
        for(int j = 0; j < S; j++) {
          delta[k] += kty[j] * Kd[k][i][j] * kty[i];
        }
        auto KtiKd = linsol.solve(Kd[k][i]);
        delta[k] -= KtiKd[i];
      }
    }
    kernel.scg(eta, delta);
    build();
  }
  D log_likelihood() const {
    D log_det = linsol.log_det();
    auto Kiy = linsol.solve(normY);
    D yky = 0;
    for(int i = 0; i < S; i++) {
      yky += normY[i] * Kiy[i];
    }
    return - log_det + yky;
  }
};
