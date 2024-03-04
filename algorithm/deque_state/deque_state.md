# deque state

[rafbillさんのツイート](https://x.com/Rafbill_pc/status/1764238239714402770?s=20)
[ahc027の提出](https://atcoder.jp/contests/ahc027/submissions/48359215)

## 状態

```cpp
struct state_phase2 {
  i32 p1[NN], ip1[NN];
  i32 p2[NN], ip2[NN];
  bool in_score[NN];
  i64 score = 0;
  void reset() {
    for(i32 i = 0; i < (i32)(nn); ++i) p1[i] = ip1[i] = i;
    for(i32 i = 0; i < (i32)(nn); ++i) p2[i] = ip2[i] = i;
    for(i32 i = 0; i < (i32)(nn); ++i) in_score[i] = 0;
    score = 0;
    for(i32 i = 0; i < (i32)(nn); ++i) add_score(i);
  }
  void remove_score(i32 x) {
    in_score[x] = 0;
    for(i32 d = 0; d < (i32)(4); ++d) if(i32 y = G[x][d]; y != -1 && in_score[y]) {
      auto ax = A[p1[ip2[x]]];
      auto ay = A[p1[ip2[y]]];
      score -= pow(ax-ay,2);
    }
  }
  void add_score(i32 x) {
    in_score[x] = 1;
    for(i32 d = 0; d < (i32)(4); ++d) if(i32 y = G[x][d]; y != -1 && in_score[y]) {
      auto ax = A[p1[ip2[x]]];
      auto ay = A[p1[ip2[y]]];
      score += pow(ax-ay,2);
    }
  }
  void swap_l(i32 i1, i32 i2) {
    i32 x1 = p2[i1], x2 = p2[i2];
    remove_score(x1);
    remove_score(x2);
    i32 j1 = p1[i1], j2 = p1[i2];
    p1[i1] = j2; p1[i2] = j1;
    ip1[j2] = i1; ip1[j1] = i2;
    add_score(x1);
    add_score(x2);
  }
  void swap_r(i32 i1, i32 i2) {
    i32 x1 = p2[i1], x2 = p2[i2];
    remove_score(x1);
    remove_score(x2);
    i32 j1 = p2[i1], j2 = p2[i2];
    p2[i1] = j2; p2[i2] = j1;
    ip2[j2] = i1; ip2[j1] = i2;
    add_score(x1);
    add_score(x2);
  }
  void swap_l_fast(i32 i1, i32 i2, i64 old_score) {
    i32 j1 = p1[i1], j2 = p1[i2];
    p1[i1] = j2; p1[i2] = j1;
    ip1[j2] = i1; ip1[j1] = i2;
    score = old_score;
  }
  void swap_r_fast(i32 i1, i32 i2, i64 old_score) {
    i32 j1 = p2[i1], j2 = p2[i2];
    p2[i1] = j2; p2[i2] = j1;
    ip2[j2] = i1; ip2[j1] = i2;
    score = old_score;
  }
};
```
## 置換について頭がなさすぎるのでまとめる

`p`が置換後の順列、`ip`が置換後の順列の各要素のindex(つまり、`p`の逆写像)
最初は`p[x] = x`、つまり順列の値は元の位置を表す、つまり
- `p[i]`: 今`i`にいる要素の値 = 今`i`にいる要素の元の位置
- `ip[i]`: 値`i`の現在の位置 = 元の位置が`i`である値の現在の位置

```
管理する置換が進む向き
------>    <------
位置に関する写像の向き
  p1         p2
<------    ------>
       L  R
------>    <------
  ip1        ip2
```

- `p1[ip2[x]]`: 今`x`にいる要素の元の位置
- `A[p1[ip2[x]]]`: 今`x`にいる要素のAの値

## 遷移

```cpp
while(!PR.empty()) {
  auto x = PR.back();
  { // add item within path
    path_item y = {Gdisc[x[0]][Gdist_sample[x[0]].sample()],Gdisc[x[1]][Gdist_sample[x[1]].sample()]};
    if(y[0] != y[1]) {
      i32 new_dist = cur_dist;
      if(!PL.empty()) new_dist -= dist_path_item(x, PL.back());
      if(!PL.empty()) new_dist += dist_path_item(y, PL.back());
      new_dist += dist_path_item(x,y);
      if(new_dist < dist_limit) {
        S.swap_l(y[0],y[1]);
        if(accept()) {
          naccept1 += 1;
          cur_dist = new_dist;
          cur_score = S.score;
          PL.emplace_back(y);
          // debug(cur_dist, cur_score);
        }else{
          S.swap_l_fast(y[0],y[1],cur_score);
        }
      }
    }
  }
  // try to remove item x
  PR.pop_back(); S.swap_r(x[0],x[1]);
  i32 new_dist = cur_dist;
  if(!PR.empty()) new_dist -= dist_path_item(x, PR.back());
  if(!PL.empty()) new_dist -= dist_path_item(x, PL.back());
  if(!PR.empty() && !PL.empty()) new_dist += dist_path_item(PL.back(), PR.back());
  if(accept()) {
    naccept2 += 1;
    cur_dist = new_dist;
    cur_score = S.score;
    // debug(cur_dist, cur_score);
  }else{
    // try to replace item x by 1 item
    path_item y;
    auto r = rng.random32(10);
    if(r <= 2) y = {Gdisc[x[0]][Gdist_sample[x[0]].sample()],Gdisc[x[1]][Gdist_sample[x[1]].sample()]};
    else if(r <= 5) y = {x[0],Gdisc[x[1]][Gdist_sample[x[1]].sample()]};
    else if(r <= 8) y = {Gdisc[x[0]][Gdist_sample[x[0]].sample()], x[1]};
    else if(r == 9) y = {(i32)rng.random32(nn), (i32)rng.random32(nn)};
    new_dist = cur_dist;
    if(!PR.empty()) new_dist -= dist_path_item(x, PR.back());
    if(!PR.empty()) new_dist += dist_path_item(y, PR.back());
    if(!PL.empty()) new_dist -= dist_path_item(x, PL.back());
    if(!PL.empty()) new_dist += dist_path_item(y, PL.back());
    if(new_dist < dist_limit) {
      S.swap_l(y[0], y[1]);
      if(accept()) {
        naccept3 += 1;
        cur_dist = new_dist;
        cur_score = S.score;
        PL.emplace_back(y);
        // debug(cur_dist, cur_score);
      }else{
        S.swap_l_fast(y[0],y[1],0);
        PL.push_back(x); S.swap_l_fast(x[0],x[1],cur_score);
      }
    }else{
      PL.push_back(x); S.swap_l_fast(x[0],x[1],cur_score);
    }
  }
}
{ // add item at end
  path_item y;
  if(PL.empty()) y = {(i32)rng.random32(nn),(i32)rng.random32(nn)};
  else { auto x = PL.back(); y = {Gdisc[x[0]][Gdist_sample[x[0]].sample()],Gdisc[x[1]][Gdist_sample[x[1]].sample()]};}
  if(y[0] != y[1]) {
    i32 new_dist = cur_dist;
    if(!PL.empty()) {
      new_dist += dist_path_item(y, PL.back());
    }
    if(new_dist < dist_limit) {
      S.swap_l(y[0],y[1]);
      if(accept()) {
        naccept4 += 1;
        cur_dist = new_dist;
        cur_score = S.score;
        PL.emplace_back(y);
        // debug(cur_dist, cur_score);
      }else{
        S.swap_l_fast(y[0],y[1],cur_score);
      }
    }
  }
}
while(!PL.empty()) {
  auto x = PL.back(); PL.pop_back(); S.swap_l_fast(x[0],x[1],cur_score);
  PR.push_back(x); S.swap_r_fast(x[0],x[1],cur_score);
}
```

1. PRの左側から要素xを取り出す
2. 要素yをPLの右に入れる遷移を試みる(add item with in path)
3. 要素xを消す遷移を試みる(try to remove item x)
4. 要素xを新しい要素yに入れ替える遷移を試みる(try to replace item x by 1 item) このとき、PLに入れる
5. 4, 5の操作でPRから一つずつ要素がPLに移されるので、PRがなくなるまで続ける
6. PLにすべて詰まったものをPRに戻す

これで遷移が定数時間で計算できる
