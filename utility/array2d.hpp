#include <array>
#include <cassert>

template<class T, int H, int W>
struct array2d {
  std::array<T, H * W> a;
  void fill(const T& t) {
    a.fill(t);
  }
  const T& at(int i, int j) const {
    return a[i * W + j];
  }
  T& at(int i, int j) {
    return a[i * W + j];
  }
};
