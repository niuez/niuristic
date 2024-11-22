# AHC007 - Online MST

[問題文](https://atcoder.jp/contests/ahc007/tasks/ahc007_a)

## 手法1 2位相当

ローカルにテストケースを生やしてMonteCarloする

Optunaでテストケースを生成するときのパラメータを変化させてよいスコアを取る、みたいな話がある https://blog.knshnb.com/posts/ahc007-optuna/

OptunaでWilcoxonPrunerを使うスクリプトを実装した。
