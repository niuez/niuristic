#include <vector>
#include <iostream>
#include <set>
#include <algorithm>
#include <queue>

std::vector<std::pair<int, int>> delta = {
  {1, 0},
  {-1,0},
  {0, 1},
  {0,-1}
};

struct State {
  int H;
  int W;
  int x, y;
  std::vector<std::vector<int>> field;
  std::vector<std::vector<int>> cnt;
  int steps = 0;
  int max_steps;
  int sum = 0;

  bool is_valid_action(int d) const {
    int nx = x + delta[d].first;
    int ny = y + delta[d].second;
    return 0 <= nx && nx < H && 0 <= ny && ny < W && field[nx][ny] != -1;
  }

  void forward(int action) {
    int nx = x + delta[action].first;
    int ny = y + delta[action].second;
    sum += cnt[nx][ny] == 0 ? field[nx][ny] : 0;
    cnt[nx][ny]++;
    x = nx;
    y = ny;
    steps++;
  }

  void backward(int action) {
    int nx = x - delta[action].first;
    int ny = y - delta[action].second;
    steps--;
    cnt[x][y]--;
    sum -= cnt[x][y] == 0 ? field[x][y] : 0;
    x = nx;
    y = ny;
  }

  bool is_done() const {
    return steps >= max_steps;
  }

  int score() const {
    return sum;
  }
};

const int max_step = 7;
const State initial_state = {
  5,
  5,
  1, 2,
  {
    {1, 5, 8, 0, 1},
    {2,-1, 0,-1, 0},
    {6, 0, 4, 5, 7},
    {5,-1, 1,-1, 3},
    {2, 7, 0, 1, 6}
  },
  std::vector<std::vector<int>>(5, std::vector<int>(5)),
  0,
  max_step,
  0,
};

struct EulerTourEdge {
  bool is_down;
  bool is_leaf;
  int action;
  int score;
};

struct EulerTourTree {
  std::vector<EulerTourEdge> edges;
  std::vector<int> leaf_scores;
};

int sol_score = -1e9;
std::vector<int> sol_actions;

void extend_euler_tree(const State& init_state, const EulerTourTree& cur_tree, EulerTourTree& next_tree, int low_score, int high_score) {
  //std::cerr << "---" << std::endl;
  State state = init_state;

  std::vector<int> down_actions;
  int downed = 0;
  for(int ei = 0; ei < cur_tree.edges.size(); ei++) {
    auto& e = cur_tree.edges[ei];
    //std::cerr << e.is_down << " " << e.is_leaf << " " << " " << e.action << " " << e.score << std::endl;
    if(e.is_down) {
      down_actions.push_back(e.action);
      state.forward(e.action);
      if(state.is_done()) {
        int score = state.score();
        if(sol_score < score) {
          std::cerr << score << std::endl;
          for(int i = 0; i < state.H; i++) {
            for(int j = 0; j < state.W; j++) {
              std::cerr << state.cnt[i][j] << " \n"[j + 1 == state.W];
            }
          }
          sol_score = score;
          sol_actions = down_actions;
        }
      }
    }
    else { // up
      if(e.is_leaf && !state.is_done() && low_score <= e.score && e.score <= high_score) {
        for(int act = 0; act < delta.size(); act++) {
          if(state.is_valid_action(act)) {
            // completion edge for new leaf
            for(; downed < down_actions.size(); downed++) {
              next_tree.edges.push_back( EulerTourEdge {
                  .is_down = true,
                  .is_leaf = false,
                  .action = down_actions[downed],
                  .score = 0,
              });
            }
            state.forward(act);
            int score = state.score();
            next_tree.edges.push_back( EulerTourEdge {
                .is_down = true,
                .is_leaf = false,
                .action = act,
                .score = 0,
            });
            next_tree.edges.push_back( EulerTourEdge {
                .is_down = false,
                .is_leaf = true,
                .action = 0,
                .score = score,
            });
            next_tree.leaf_scores.push_back(score);
            state.backward(act);
          }
        }
      }
      if(down_actions.empty()) {
        next_tree.edges.push_back( EulerTourEdge {
            .is_down = false,
            .is_leaf = false,
            .action = 0,
            .score = 0,
        });
        return;
      }
      state.backward(down_actions.back());
      if(downed == down_actions.size()) {
        next_tree.edges.push_back( EulerTourEdge {
            .is_down = false,
            .is_leaf = false,
            .action = 0,
            .score = 0,
        });
        downed--;
      }
      down_actions.pop_back();
    }
  }
}

void beam_search(const State& init_state, int max_step) {
  const int beam_width = 5;
  EulerTourTree cur_tree;
  EulerTourTree next_tree;
  cur_tree.edges.push_back( EulerTourEdge {
      .is_down = 0,
      .is_leaf = 1,
      .action = 0,
      .score = 0,
  });
  cur_tree.leaf_scores.push_back(0);
  for(int step = 0; step <= max_step; step++) {
    auto& score = cur_tree.leaf_scores;
    int w = std::min(beam_width, (int)score.size());
    std::partial_sort(score.begin(), score.begin() + w, score.end(), std::greater<int>());
    next_tree.edges.clear();
    next_tree.leaf_scores.clear();
    std::cerr << step << " " << score[score.size() - 1] << " " << score[score.size() - w] << std::endl;
    extend_euler_tree(init_state, cur_tree, next_tree, score[w - 1], score[0]);
    std::swap(cur_tree, next_tree);
  }
  std::cerr << sol_score << std::endl;
  State state = init_state;
  for(int i = 0; i < sol_actions.size(); i++) {
    state.forward(sol_actions[i]);
    std::cerr << sol_actions[i] << " \n"[i + 1 == sol_actions.size()];
  }
  std::cerr << state.score() << std::endl;
}

int main() {
  beam_search(initial_state, max_step);
}
