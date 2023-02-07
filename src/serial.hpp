#include <iostream>
#include <iomanip>
#include <iterator>
#include <vector>
#include <sstream>

struct serializer {
  static constexpr int precision = 6;
  std::ostream& os;
  serializer(std::ostream& o): os(o) {}

  serializer& operator<<(bool n) { os << n << ' '; return *this; }
  serializer& operator<<(short n) { os << n << ' '; return *this; }
  serializer& operator<<(unsigned short n) { os << n << ' '; return *this; }
  serializer& operator<<(int n) { os << n << ' '; return *this; }
  serializer& operator<<(unsigned int n) { os << n << ' '; return *this; }
  serializer& operator<<(long n) { os << n << ' '; return *this; }
  serializer& operator<<(unsigned long n) { os << n << ' '; return *this; }
  serializer& operator<<(long long n) { os << n << ' '; return *this; }
  serializer& operator<<(float n) { os << std::fixed << std::setprecision(precision) << n << ' '; return *this; }
  serializer& operator<<(double n) { os << std::fixed << std::setprecision(precision) << n << ' '; return *this; }
  serializer& operator<<(long double n) { os << std::fixed << std::setprecision(precision) << n << ' '; return *this; }
  template<class T>
  auto operator<<(const T& v) -> decltype(
      std::begin(std::declval<T&>()) != std::end(std::declval<T&>()),
      ++std::declval<decltype(std::begin(std::declval<T&>()))&>(),
      void(*std::begin(std::declval<T&>())),
      std::declval<serializer&>()) {
    std::size_t d = std::distance(std::begin(v), std::end(v));
    os << "[ " << d << " | ";
    for(const auto& e: v) {
      (*this) << e;
    }
    os << " ] ";
    return *this;
  }
  serializer& operator<<(std::ostream& (*pf)(std::ostream&)) { os << pf; return *this; }
  serializer& operator<<(std::ios_base& (*pf)(std::ios_base&)) { os << pf; return *this; }
};

struct deserializer {
  std::istream& is;
  deserializer(std::istream& i): is(i) {}
  deserializer& operator>>(bool& n) { is >> n; return *this; }
  deserializer& operator>>(short& n) { is >> n; return *this; }
  deserializer& operator>>(unsigned short& n) { is >> n; return *this; }
  deserializer& operator>>(int& n) { is >> n; return *this; }
  deserializer& operator>>(unsigned int& n) { is >> n; return *this; }
  deserializer& operator>>(long& n) { is >> n; return *this; }
  deserializer& operator>>(unsigned long& n) { is >> n; return *this; }
  deserializer& operator>>(long long& n) { is >> n; return *this; }
  deserializer& operator>>(float& n) { is >> n; return *this; }
  deserializer& operator>>(double& n) { is >> n; return *this; }
  deserializer& operator>>(long double& n) { is >> n; return *this; }
  template<class T>
  auto operator>>(T& v) -> decltype(
      std::begin(std::declval<T&>()) != std::end(std::declval<T&>()),
      ++std::declval<decltype(std::begin(std::declval<T&>()))&>(),
      void(*std::begin(std::declval<T&>())),
      std::declval<deserializer&>()) {
    static std::string ign;
    std::size_t d;
    is >> ign >> d >> ign;
    std::vector<typename T::value_type> vs;
    vs.resize(d);
    for(std::size_t i = 0; i < d; i++) {
      (*this) >> vs[i];
    }
    is >> ign;
    v = T(std::make_move_iterator(vs.begin()), std::make_move_iterator(vs.end()));
    return *this;
  }
};

