# 焼きなまし 遷移高速化

## 高速化するべき？

まず、過度な高速化するべきかをちゃんと判断する。実行時間を伸ばすなど。

## 高速化テク: 遷移チェックの計算量をなるべく小さくする

受理の回数が少ない場合、遷移チェックの計算を軽くするために受理時の再計算を頑張る

- [AHC041解説 - Speaker Deck](https://speakerdeck.com/terryu16/ahc041jie-shuo)
- [AHC041 7位解法｜niuez](https://sizu.me/niuez/posts/snzdtkd7a6r2)


## 高速化テク: 償却テク

自分が勝手に呼んでいる。例えば、遷移候補$`O(N)`$個を一気に検証するアルゴリズムが$`O(N)`$の計算量で動作するとき、
各遷移を$`O(1)`$で処理できている気分になりませんか、という意味。

自分が知っている償却テクはこれらで、どちらも償却テクとして認識したほうが認知範囲が広げられそうだと考えている。

- 貪欲遷移
    - 遷移候補$`O(N)`$個からスコアの差分が最小になる遷移を1つ見つけるのを$`O(N)`$でやる
    - AHC特有だが、2secという実行時間ならある程度貪欲に遷移したほうが良いことがある
- Deque State
    - 状態をdeque=queue2つとして持ち、2つのqueueの中間地点を1つ進めたり、中間地点になにか挿入したりすることを$`O(1)`$でやることで、候補$`O(N)`$個を全体$`O(N)`$で済ます


## 貪欲遷移


- [AHC037 Sodaの自分のテスター解(2位)](https://atcoder.jp/contests/ahc037/submissions/57824394)

この回は貪欲勢が多かったが、実はRafbillさんと自分は焼きなまし。

状態をN個の葉が「最終的に作りたい頂点」を表す全二分木とする。 この木の節(子供をちょうど２つ持つノード)は、子供に点を振り分けるためのペナルティが小さい座標$`(x, y) = (min(x1, x2), min(y1, y2))`$に点を配置する意味を持つ。

このように状態を持つと、「ランダムに部分木を選び、他の子とする」遷移を考えることができる。愚直に実装するとスコア計算や受理に$`O(N)`$かかってしまう。

しかし、どこに挿入するのが最善かを木DPで計算することにすると、償却テクになる。

自分の提出コードでは、
- `until[i]`: ノード`i`の部分木で、まだ挿入をすませていない場合のスコアの最小値
- `merge[i]`: ノード`i`の部分木で、すでに挿入をすませた場合のスコアの最小値と挿入先の組
としている。

```rust
let from = mt.gen_range(1..state.ett.len());
let moved_l = from;
let moved_r = from + state.ett[from].sz;
let mut delete_v = !0;
let mut until = vec![(!0, P::default()); state.ett.len()];
let mut merged = vec![(!0, P::default(), !0); state.ett.len()];
for i in (0..state.ett.len()).rev() {
    if moved_l <= i && i < moved_r { continue; }
    if state.ett[i].leaf {
        until[i] = (0, state.ett[i].p.clone());
        let (a, c) = state.ett[i].p.ancestor(&state.ett[from].p);
        merged[i] = (c + state.ett[from].cost, a, i);
    }
    else {
        let l = i + 1;
        let r = i + 1 + state.ett[l].sz;
        if r == from {
            until[i] = until[l].clone();
            merged[i] = merged[l].clone();
            delete_v = i;
        }
        else if l == from {
            until[i] = until[r].clone();
            merged[i] = merged[r].clone();
            delete_v = i;
        }
        else {
            let (a, c) = until[l].1.ancestor(&until[r].1);
            until[i] = (c + until[l].0 + until[r].0, a);
            let (a, c) = until[i].1.ancestor(&state.ett[from].p);
            merged[i] = (c + until[i].0 + state.ett[from].cost, a, i);
            let (a, c) = until[l].1.ancestor(&merged[r].1);
            merged[i] = merged[i].clone().min((c + until[l].0 + merged[r].0, a, merged[r].2));
            let (a, c) = merged[l].1.ancestor(&until[r].1);
            merged[i] = merged[i].clone().min((c + merged[l].0 + until[r].0, a, merged[l].2));
        }
    }
}
let to = merged[0].2;
let score = merged[0].0 + merged[0].1.x + merged[0].1.y;
return Neiborhood { from, delete_v, to, score };
```

## Deque State

Rafbillさんの提出コードを読んだときのメモ

[Rafbillさんのツイート](https://x.com/Rafbill_pc/status/1764238239714402770?s=20)
[ahc027の提出](https://atcoder.jp/contests/ahc027/submissions/48359215)

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

置換について頭がなさすぎるのでまとめる

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

