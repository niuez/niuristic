#include <cstdint>
#include <array>
#include <iostream>
#include <string>

namespace niu {
  template<int N, class Uint = std::uint_fast64_t, int wl = 6>
  struct bitset {
    constexpr static Uint W = 1 << wl;
    constexpr static Uint L = (N + W - 1) / W;
    std::array<Uint, L> bits = {};

    struct reference {
      Uint& b;
      Uint j;
      constexpr reference& operator=(bool x) {
        b = (b & ~(Uint(1) << j)) | Uint(x) << j;
        return *this;
      }
      constexpr operator bool() const {
        return b & (Uint(1) << j);
      }
    };

    constexpr reference operator[](Uint i) {
      return reference { .b = bits[i >> wl], .j = i & (W - 1) };
    }
    constexpr bool operator[](Uint i) const {
      return bits[i >> wl] & (Uint(1) << (i & (W - 1)));
    }

    constexpr bitset<N, Uint, wl>& operator&=(const bitset<N, Uint, wl>& b) {
      for(int i = 0; i < L; i++) {
        bits[i] &= b.bits[i];
      }
      return *this;
    }
    constexpr bitset<N, Uint, wl>& operator|=(const bitset<N, Uint, wl>& b) {
      for(int i = 0; i < L; i++) {
        bits[i] |= b.bits[i];
      }
      return *this;
    }
    constexpr bitset<N, Uint, wl> operator&(const bitset<N, Uint, wl>& b) const { bitset<N, Uint, wl>(*this) &= b; }
    constexpr bitset<N, Uint, wl> operator|(const bitset<N, Uint, wl>& b) const { bitset<N, Uint, wl>(*this) |= b; }
    constexpr bool operator==(const bitset<N, Uint, wl>& b) const { return bits == b.bits; }
    constexpr bool operator!=(const bitset<N, Uint, wl>& b) const { return bits != b.bits; }

    constexpr Uint count() const {
      Uint ans = 0;
      for(int i = 0; i < L; i++) {
        ans += __builtin_popcountll(bits[i]);
      }
      return ans;
    }

    constexpr Uint find_first() const {
      for(int i = 0; i < L; i++) {
        if(bits[i]) {
          return (i << wl) + __builtin_ctzll(bits[i]);
        }
      }
      return N;
    }

    constexpr Uint find_next(Uint x) const {
      x++;
      Uint l = x >> wl;
      Uint b = bits[l] & ((~Uint(1)) << (x & (W - 1)));
      if(b) {
        return (l << wl) + __builtin_ctzll(b);
      }
      for(int i = l + 1; i < L; i++) {
        if(bits[i]) {
          return (i << wl) + __builtin_ctzll(bits[i]);
        }
      }
      return N;
    }

    std::string to_string() const {
      std::string ans;
      ans.resize(N);
      for(int i = 0; i < N; i++) {
        ans[i] = char((int)(*this)[i] + '0');
      }
      return ans;
    }

    friend std::ostream& operator<<(std::ostream& os, const bitset<N, Uint, wl>& b) {
      return os << b.to_string();
    }
  };
}
