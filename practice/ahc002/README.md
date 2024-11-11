# AHC002 - Walking on Tiles

[問題文](https://atcoder.jp/contests/ahc002/tasks/ahc002_a)

## 手法1

takumi152さんの解法とツイートを参考にした
thunderさんの解法説明も詳しい https://qiita.com/thun-c/items/ecd438fde4d237b1f7bc

焼きなまし
初期解: 0.5秒間DFSして最もスコアが高かったもの
遷移: 2-20の長さの部分列を破壊しDFSで構築、`0.000_020sec`探索を行う(巨大近傍法?)
採用するかどうかはDFSの途中で、採用した時の差分が`Temperature * ln(rand(0.0..1.0))`を超えているかどうかで判定する。
超えていたら、それを採用しDFSを中断する。(焼きなましの閾値判定)


これで3位相当
