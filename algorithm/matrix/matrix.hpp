#pragma once
#include <vector>
#include <cassert>
#include <iostream>
#include <numeric>

template<class T>
struct Matrix: public std::vector<std::vector<T>> {
  int H;
  int W;
  Matrix() {}
  Matrix(int H, int W, T t = T())
    : H(H),
      W(W),
      std::vector<std::vector<T>>(H, std::vector<T>(W, t)) {}
  Matrix(std::vector<std::vector<T>> vec)
      : std::vector<std::vector<T>>(vec) {
        this->H = this->size();
        this->W = this->empty() ? 0 : (*this)[0].size();
      }
};
