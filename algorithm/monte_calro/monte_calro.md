# Monte Calro method

- 新ジャッジコンテスト
- AHC029(ynasuさんのコードがわかりやすい)

行動を繰り返して報酬を得るゲーム系

## 貪欲法

そのターンに取れる行動について**行動価値**を計算し、価値が最も高いものを選択し続ける

```cpp
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
```

行動価値
- その行動を取った瞬間に得られる報酬
- その行動を取らなかった場合に将来に得そうな損失(`newjudge_2304`)


## MC method

今の盤面から取れる行動を一つ行った後、
ゲーム最終 or nターン後の盤面まで進めた(playout)ときの**状態価値**の期待値を計算する手法

- ゲームの進め方は貪欲法に基づく場合が多い
- ランダム性を含むゲームの場合、何ケースも試す

```cpp
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
```

状態価値
- ゲームを進める間に得た報酬
- これから先ゲームを進めた時に得そうな損失

## 関連ワード?

- 順モンテカルロ 周期性?
