#include <compare>
#include <array>
#include <utility>

struct Pos {
  int x;
  int y;
  constexpr Pos(int x = 0, int y = 0): x(x), y(y) {}
  auto operator<=>(const Pos& p) const = default;

  Pos& operator+=(const Pos& p) {
    x += p.x;
    y += p.y;
    return *this;
  }
  Pos& operator-=(const Pos& p) {
    x -= p.x;
    y -= p.y;
    return *this;
  }

  Pos operator+(const Pos& p) const { return Pos(*this) += p; }
  Pos operator-(const Pos& p) const { return Pos(*this) -= p; }
};

struct PositionValidator {
  int xl, xr, yl, yr;

  PositionValidator(int xl, int xr, int yl, int yr)
    : xl(xl), xr(xr), yl(yl), yr(yr) {}

  bool operator()(const Pos& p) const {
    return xl <= p.x && p.x < xr && yl <= p.y && p.y < yr;
  }
};

struct PosAdj {
  constexpr static std::array<std::pair<Pos, char>, 4> ps = {
    std::pair<Pos, char> { Pos(-1, 0), 'U' },
    { Pos(+1, 0), 'D' },
    { Pos(0, -1), 'L' },
    { Pos(0, +1), 'R' },
  };

  const int id;

  constexpr PosAdj(int i): id(i) {}
  char to_char() const { return ps[id].second; }
  Pos pos() const { return ps[id].first; }
};

constexpr std::array<PosAdj, 4> adjs = { PosAdj(0), PosAdj(1), PosAdj(2), PosAdj(3) };
