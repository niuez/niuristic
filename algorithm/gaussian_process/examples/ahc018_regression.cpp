#include "../../matrix/cholesky_decomposition.hpp"
#include "../../matrix/matrix.hpp"

struct Input {
  int N;
  int W;
  int K;
  int C;

  std::vector<std::pair<int, int>> waters;
  std::vector<std::pair<int, int>> houses;
  inline int index(int i, int j) const {
    return i * N + j;
  }
};

struct InlineJudge {
  int N;
  std::vector<std::vector<int>> field;
  InlineJudge(int n, std::istream& is): N(n), field(n, std::vector<int>(n)) {
    for(int i = 0; i < n; i++) {
      for(int j = 0; j < n; j++) {
        is >> field[i][j];
      }
    }
  }
};

template<class Judge>
std::tuple<Input, Judge> get_input(std::istream& is) {
  Input in;
  is >> in.N >> in.W >> in.K >> in.C;
  
  Judge judge(in.N, is);

  in.waters.resize(in.W);
  for(int i = 0; i < in.W; i++) {
    is >> in.waters[i].first >> in.waters[i].second;
  }
  in.houses.resize(in.K);
  for(int i = 0; i < in.K; i++) {
    is >> in.houses[i].first >> in.houses[i].second;
  }
  return std::make_tuple(std::move(in), std::move(judge));
}

#include "../gaussian_process_regression.hpp"

int main() {
  using D = double;

  auto [in, judge] = get_input<InlineJudge>(std::cin);
  RBFKernel<D> rbf(0.1, 0.01, 0.001);
  const int window = 20;
  const int S = (in.N / window) * (in.N / window);

  GaussianProcessRegression gpr(rbf);
  for(int i = 0; i < in.N / window; i++) {
    for(int j = 0; j < in.N / window; j++) {
      gpr.add(
          { D(i * window) / D(in.N), D(j * window) / D(in.N) }, 
          std::sqrt(D(judge.field[i * window][j * window]))
      );
    }
  }
  gpr.build();
  for(int i = 0; i < in.N; i++) {
    for(int j = 0; j < in.N; j++) {
      std::vector<D> xd = { D(i) / D(in.N), D(j) / D(in.N) };
      auto [mu, sigma] = gpr.reg(xd);
      std::cout << std::clamp(int(mu * mu), 10, 5000) << " ";
      std::cout << std::flush;
    }
    std::cout << std::endl;
  }
}
