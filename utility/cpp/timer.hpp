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
