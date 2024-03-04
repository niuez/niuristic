#include <array>
#include <cassert>

// optimized in -O3 https://godbolt.org/z/zvqMYbhPx

template<class T, int... Size>
struct arraynd {
  constexpr static int L = (Size * ...);
  constexpr static std::array<int, sizeof...(Size)> sz = { Size... };

  template<int Dim, class Head, class... Tail>
  static inline constexpr int to_index(int x, Head h, Tail... tail) {
    return to_index<Dim + 1>(x * sz[Dim] + h, std::forward<Tail>(tail)...);
  }
  template<int Dim>
  static inline constexpr int to_index(int x) {
    return x;
  }

  std::array<T, L> a;
  void fill(const T& t) {
    a.fill(t);
  }
  template<class... Index>
  const T& operator[](Index... is) const {
    static_assert(sizeof...(Index) == sizeof...(Size));
    return a[to_index<0>(0, std::forward<Index>(is)...)];
  }
  template<class... Index>
  T& operator[](Index... is) {
    static_assert(sizeof...(Index) == sizeof...(Size));
    return a[to_index<0>(0, std::forward<Index>(is)...)];
  }
};
