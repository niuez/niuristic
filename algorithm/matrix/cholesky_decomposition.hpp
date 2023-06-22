#pragma once
#include "matrix.hpp"

// https://www.slis.tsukuba.ac.jp/~fujisawa.makoto.fu/cgi-bin/wiki/?%A5%B3%A5%EC%A5%B9%A5%AD%A1%BC%CA%AC%B2%F2
// Modified Cholesky Decomposition
// Ax = LDL^T x = b
template<class D>
struct ModifiedCholeskyDecomposition {
  int n;
  Matrix<D> A;
  std::vector<D> d;
  ModifiedCholeskyDecomposition() {}
  ModifiedCholeskyDecomposition(Matrix<D> a) {
    init(std::move(a));
  }
  void init(Matrix<D> a) {
    A = std::move(a);
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

  D log_det() const {
    D ans = 0;
    for(int i = 0; i < n; i++) {
      ans += A[i][i] + A[i][i] + d[i];
    }
    return ans;
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
