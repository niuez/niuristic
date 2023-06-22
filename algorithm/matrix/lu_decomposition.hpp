#include "matrix.hpp"

// http://www.ced.is.utsunomiya-u.ac.jp/lecture/2011/prog/p2/kadai3/no3/lu.pdf
// https://www.cspp.cc.u-tokyo.ac.jp/hanawa/class/spc2016s/sp20160614-2.pdf
// LU Decomposition: right-looking algorithm
// Ax = LUx = b
template<class D>
struct LUDecomposition {
  int n;
  Matrix<D> A;
  LUDecomposition(Matrix<D> a): A(std::move(a)) {
    assert(A.H == A.W);
    n = A.H;
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
  }

  std::vector<D> solve(std::vector<D> b) const {
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
};
