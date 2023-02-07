#include <cstdint>
#include <numeric>

struct Xor64 {
  using state_type = std::uint64_t;
  using result_type = std::uint64_t;
  state_type a;
  static constexpr result_type min() {
    return std::numeric_limits<result_type>::min();
  }
  static constexpr result_type max() {
    return std::numeric_limits<result_type>::max();
  }
  constexpr Xor64(state_type seed = 88675123): a(seed) {}
  constexpr void seed(state_type seed = 88675123) {
    a = seed;
  }
  constexpr result_type operator()() {
    a ^= a << 7;
    a ^= a >> 9;
    return a;
  }
  void discard(unsigned long long z) {
    while(z --> 0) {
      (*this)();
    }
  }
};
