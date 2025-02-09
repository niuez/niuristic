# Beam Search 高速化

## ビームサーチの状態遷移

ビームサーチ中の状態遷移では、

- 状態を遷移させコピーする
- 遷移を進める・巻き戻すを繰り返して盤面を再現する

の二つがある。後者についての勉強をする。

## Euler Tour Treeによる管理

[Rafbillさんの提出 #42958555 - TOYOTA Programming Contest 2023 Summer（AtCoder Heuristic Contest 021）](https://atcoder.jp/contests/ahc021/submissions/42958555)を参考にした。

![eet](eet.jpg)

- ビームを深くするたびにETTを拡張する
- 四角で囲まれているのは、ビーム幅で選ばれた頂点
- 丸で囲まれているのは、木の葉
- 下線は、選ばれた頂点だけをETTで表現するために必要な辺

## 高速化

候補となる遷移全てを一度ETTにする実装だとかなりのメモリが必要になるため、動作が非常に遅い。  
対策として、今まで見た候補のうち上位W個のスコアが良いものを保持するsegment treeを用意すれば、空間計算量を遷移の候補数ではなくビーム幅で抑えることができる。

[自分の実装例](../../utility/rust/src/beam_search/ett_beam.rs)

C++の実装の例: [差分更新ビームサーチライブラリの実装 (C++) - eijirouの競プロ参加記](https://eijirou-kyopro.hatenablog.com/entry/2024/02/01/115639)

