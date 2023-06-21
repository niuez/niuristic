#include <vector>
#include <cassert>
#include <iostream>
#include <numeric>


template<class T>
struct Matrix: public std::vector<std::vector<T>> {
  int H;
  int W;
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
  for(int i = 0; i < A.H; i++) {
    for(int j = 0; j < A.W; j++) {
      std::cerr << A[i][j] << " \n"[j + 1 == A.W];
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
  for(int i = 0; i < n; i++) {
    for(int j = 0; j < n; j++) {
      std::cerr << A[i][j] << " \n"[j + 1 == n];
    }
  }
  for(int i = 0; i < n; i++) {
    std::cerr << d[i] << " \n"[i + 1 == n];
  }
  // LDy = b
  for(int i = 0; i < n; i++) {
    D s = b[i];
    for(int j = 0; j < i; j++) {
      s -= A[i][j] * d[j] * b[j];
    }
    b[i] = s; // L[i][i] = 1
    std::cerr << b[i] << " \n"[i + 1 == n];
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

int main() {
  {
    Matrix<D> A = std::vector<std::vector<D>> {
      { 8, 16, 24, 32 },
        { 2, 7, 12, 17 },
        { 6, 17, 32, 59 },
        { 7, 22, 46, 105 }
    };
    std::vector<D> b = { 160, 70, 198, 291 };
    std::cerr << A.H << " " << A.W << std::endl;
    auto x = linsolve(A, b);
    for(int i = 0; i < x.size(); i++) {
      std::cout << x[i] << " \n"[i + 1 == x.size()];
    }
    for(int i = 0; i < A.H; i++) {
      double s = 0;
      for(int j = 0; j < A.W; j++) {
        s += A[i][j] * x[j];
      }
      std::cout << s << " \n"[i + 1 == A.H];
    }
  }
  {
    Matrix<D> A = std::vector<std::vector<D>> {
      { 1, 2, 3 },
      { 2, 5, 6 },
      { 3, 6, 7 },
    };
    std::vector<D> b(3);
    std::vector<D> z = { 1, 2, 3 };
    for(int i = 0; i < A.H; i++) {
      double s = 0;
      for(int j = 0; j < A.W; j++) {
        s += A[i][j] * z[j];
      }
      b[i] = s;
      std::cout << s << " \n"[i + 1 == A.H];
    }
    std::cerr << A.H << " " << A.W << std::endl;
    auto x = symmetric_linsolve(A, b);
    for(int i = 0; i < x.size(); i++) {
      std::cout << x[i] << " \n"[i + 1 == x.size()];
    }
    for(int i = 0; i < A.H; i++) {
      double s = 0;
      for(int j = 0; j < A.W; j++) {
        s += A[i][j] * x[j];
      }
      std::cout << s << " \n"[i + 1 == A.H];
    }
  }
}
