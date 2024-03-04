#include <cstdint>
#include <numeric>

struct Xor128 {
  using state_type = std::uint32_t;
  using result_type = std::uint32_t;
  state_type x = 123456789, y = 362436039, z = 521288629, w = 88675123;
  constexpr static double INV_MAX = 1.0 / 0xFFFFFFFF;

  static constexpr result_type min() {
    return std::numeric_limits<result_type>::min();
  }
  static constexpr result_type max() {
    return std::numeric_limits<result_type>::max();
  }

  constexpr Xor128(state_type seed = 88675123): w(seed) {}
  constexpr void seed(state_type seed = 88675123) {
    w = seed;
  }
  constexpr result_type operator()() {
    state_type t = x ^ (x << 11);
    x = y, y = z, z = w;
    return w = (w ^ (w >> 19)) ^ (t ^ (t >> 8));
  }
  void discard(unsigned long long z) {
    while(z --> 0) {
      (*this)();
    }
  }

  // [0, a)
  constexpr result_type rand_int(state_type a) {
    return ((std::uint64_t) (*this)() * a) >> 32;
  }
  
  // [a, b]
  constexpr result_type rand_int(state_type a, state_type b) {
    return rand_int(b - a + 1) + a;
  }
  
  // [0, 1]
  constexpr double prob() {
    return (*this)() * INV_MAX;
  }
  
  // [a, b]
  constexpr double rand_double(double a, double b) {
    return prob() * (b - a) + a;
  }
};
