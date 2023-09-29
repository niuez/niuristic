#include <vector>

template<class T>
struct fast_int_distribution {
  const T a;
  const T l;
  fast_int_distribution(T a, T b): a(a), l(b - a + 1) {}
  template<class G>
  T operator()(G& g) const {
    return a + g() % l;
  }
};

struct discrate_probability_distribution {
  using T = int;
  std::vector<T> acc;
  discrate_probability_distribution(const std::vector<T>& prob) {
    acc.resize(prob.size(), 0);
    T now = 0;
    for(int i = 0; i < prob.size(); i++) {
      now += prob[i];
      acc[i] = now;
    }
  }

  template<class URGB>
    int operator()(URGB& g) const {
      double v = fast_int_distribution<int>(0, acc.back() - 1)(g);
      return std::lower_bound(acc.begin(), acc.end(), v) - acc.begin();
    }
};
