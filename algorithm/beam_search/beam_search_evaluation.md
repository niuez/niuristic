# Beam Search 評価関数

(個人的に)Beam Searchを使おうと至る根拠には、それぞれ逆方向な思考方法が存在しているようなイメージがある

- 貪欲 -> Beam Search
    - 強い貪欲解が存在するため、その探索範囲を広げるアプローチとしてのBeam Search
    - Pyramid Sorting: [AHC021の超強い貪欲](https://atcoder.jp/contests/ahc021/editorial)
    - Mod Stamp: [AHC032 解説](https://img.atcoder.jp/ahc032/editorial.pdf?_gl=1*10fi4r4*_ga*NzIxNDc0ODE0LjE3MzIxNzQ2NzY.*_ga_RC512FD18N*MTczOTA3NDI5OS44NC4xLjE3MzkwNzQzMzIuMC4wLjA.)
    - Christmas Tree Cutting: [AHC041 参加記・本番 1 位解法の説明 - ひとなので](https://rsm9.hatenablog.com/entry/2025/01/22/213923)
- 全探索 -> Beam Search
    - ある盤面から終了状態に到達するまでのペナルティのおおよその下界が計算できる
    - Lucky Words: [AHC028解説 - Speaker Deck](https://speakerdeck.com/terryu16/ahc028jie-shuo)
    - Crane: [AHC033 参加記 - eijirouの競プロ参加記](https://eijirou-kyopro.hatenablog.com/entry/2024/05/30/110801)
    - 節分: [AHC042の自分の提出](https://atcoder.jp/contests/ahc042/submissions/62339270)

## 2つ目に関する自分の解釈メモ

DFSで問題を解くことを考える。DFSの状態を $s$、その状態からDFSで得ることができる終端状態の集合を $T(s)$、終端状態に対する目的関数を $f(t)$とする。
貪欲によって最適解を得るためには、状態$s$から遷移できる状態 $s'$であって、 $m(s') := min_{t \in T(s')} f(t)$が最も小さいものを選び続ける必要がある。
 $m(s')$自体、またはそれと同じ大小関係を持つ関数 $g(s')$を計算することができればよい。

大抵の場合、 $g(s')$を得るのは困難。そこで、 $g(s')$に似た大小関係を持つ $h(s')$を考え、各遷移の段階で $W$個の良い解を持つ。これがビームサーチ。
$h(s')$にあるとうれしい性質は、

- $g(s')$の大小関係とのブレが小さい
- $min_{t \in T(s')} f(t)$の下界とする。ただし、下界が達成可能であるほどよい(ブレが小さいと同義)。

## 具体的な評価関数の設計
