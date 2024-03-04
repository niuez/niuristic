#include <array>
#include <cassert>

//#define DEBUG_FIXED_QUEUE

template<class T, int N>
struct fixed_queue {
  std::array<T, N> a;
  int sz = 0;
  int l = 0;
  int r = 0;
  
  bool empty() const { return sz == 0; }
  std::size_t size() const { return sz; }
  const T& front() const {
#ifdef DEBUG_FIXEDQUEUE
    assert(sz > 0);
#endif
    return a[l];
  }
  T& front() {
#ifdef DEBUG_FIXEDQUEUE
    assert(sz > 0);
#endif
    return a[l];
  }
  void push(const T& t) {
#ifdef DEBUG_FIXEDQUEUE
    assert(sz < N);
#endif
    a[r++] = t;
    sz++;
    if(r >= N) r = 0;
  }
  void push(T&& t) {
#ifdef DEBUG_FIXEDQUEUE
    assert(sz < N);
#endif
    a[r++] = std::move(t);
    sz++;
    if(r >= N) r = 0;
  }
  void pop() {
#ifdef DEBUG_FIXEDQUEUE
    assert(sz > 0);
#endif
    l++; sz--;
    if(l >= N) l = 0;
  }
};
