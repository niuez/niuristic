#include <iostream>
#include <vector>
#include <random>
#include <algorithm>

#include <chrono>

struct Timer {
  std::chrono::high_resolution_clock::time_point st;
  Timer() { st = now(); }
  std::chrono::high_resolution_clock::time_point now() { return std::chrono::high_resolution_clock::now(); }
  std::chrono::milliseconds::rep span() {
    auto ed = now();
    return std::chrono::duration_cast<std::chrono::milliseconds>(ed - st).count();
  }
};

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

constexpr int D = 365;
constexpr int K = 26;
struct Judge {
  std::array<int, K> initial_c() {
    std::array<int, K> c;
    int d_;
    std::cin >> d_;
    for(int k = 0; k < K; k++) {
      std::cin >> c[k];
    }
    return c;
  }

  void select_kind(int k) {
    std::cout << k + 1 << std::endl;
  }

  std::array<int, K> read_contest_scores() {
    std::array<int, K> s;
    for(int k = 0; k < K; k++) {
      std::cin >> s[k];
    }
    return s;
  }
};

Judge judge;
Timer timer;
constexpr double TL = 1900;


constexpr int simulation_days = 25;
constexpr int greedy_choices = 3;

struct Solver {
  std::array<int, K> c;
  std::array<std::array<int, K>, D> s;
  std::array<double, K> mu;
  std::array<double, K> sigma;

  std::array<double, D + 1> pow_table;
  Xor128 mt;

  Solver() {
    for(int d = 0; d <= D; d++) {
      pow_table[d] = std::pow(d, 1.5);
    }
  }

  double profit(int day, int k, const std::array<int, K>& last) const {
    double future_cost = 1.3 * c[k] * pow_table[day - last[k]] - 0.6 * mu[k] - 0.95 * sigma[k];
    constexpr int first_phase = 9;
    if (day < first_phase) {
      future_cost *= (double)day / first_phase;
    }
    constexpr int last_phase = 12;
    if (D - day < last_phase) {
      future_cost *= (double)(D - day) / last_phase;
    }
    return s[day][k] + future_cost;
  }

  int choose(int day, const std::array<int, K>& last) const {
    std::pair<double, int> max = { -1e18, -1 };
    for(int k = 0; k < K; k++) {
      max = std::max(max, { profit(day, k, last), k });
    }
    return max.second;
  }


  double playout(int start_day, int start_k, std::array<int, K> last) const {
    double score = 0;
    last[start_k] = start_day;
    score += s[start_day][start_k];
    for(int k = 0; k < K; k++) {
      score -= (start_day - last[k]) * c[k];
    }
    int end = std::min(D, start_day + 1 + simulation_days);
    for(int day = start_day; day < end; day++) {
      int dk = choose(day, last);

      last[dk] = day;
      score += s[day][dk];
      for(int k = 0; k < K; k++) {
        score -= (day - last[k]) * c[k];
      }
    }
    double future = 0;
    for(int k = 0; k < K; k++) {
      future -= pow_table[end - last[k]] * c[k];
    }
    constexpr int last_phase = 8;
    if (D - last_phase < end) {
      future *= (double)(D - end) / last_phase;
    }

    return score + future;
  }

  double monte_carlo_method(int day, const std::array<int, K>& last) {
    std::array<std::pair<double,int>,26> profits;
    for (int k = 0; k < K; k++) {
      profits[k] = { -profit(day, k, last), k };
    }
    sort(profits.begin(), profits.end());

    if (profits[1].first - profits[0].first > 5000.0) {
      return profits[0].second;
    }
    std::array<double, greedy_choices> sum_scores;
    sum_scores.fill(0);

    int it = 0;
    while (timer.span() < TL * (day + 1) / D) {
      it++;
      for (int i = 0; i < K; ++i) {
        std::normal_distribution<> dist(mu[i], sigma[i]);
        for(int d = day + 1; d < std::min(D, day + 1 + simulation_days); d++) {
          s[d][i] = std::clamp(dist(mt), 0.0, 1e5);
        }
      }
      for (int i = 0; i < greedy_choices; ++i) {
        sum_scores[i] += playout(day, profits[i].second, last);
      }
    }
    std::cerr << day << " " << it << std::endl;
    return profits[std::max_element(sum_scores.begin(), sum_scores.end()) - sum_scores.begin()].second;
  }

  void solve() {
    c = judge.initial_c();

    std::array<int, K> last;
    last.fill(-1);

    std::array<double, K> sum1;
    std::array<double, K> sum2;
    sum1.fill(0);
    sum2.fill(0);

    for(int day = 0; day < D; day++) {
      s[day] = judge.read_contest_scores();
      for(int k = 0; k < K; k++) {
        sum1[k] += s[day][k];
        sum2[k] += s[day][k] * s[day][k];
        mu[k] = sum1[k] / (day + 1);
        sigma[k] = std::sqrt(sum2[k] / (day + 1) - mu[k] * mu[k]);
      }
      int k = monte_carlo_method(day, last);
      last[k] = day;
      judge.select_kind(k);
    }
  }
};

int main() {
  Solver solver;
  solver.solve();
}
