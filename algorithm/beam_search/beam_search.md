# Beam Search

## ビームサーチの状態遷移

ビームサーチ中の状態遷移では、

- 状態を遷移させコピーする
- 遷移を進める・巻き戻すを繰り返して盤面を再現する

の二つがある。後者についての勉強をする。

## Euler Tour Treeによる管理

[Rafbillさんの提出 #42958555 - TOYOTA Programming Contest 2023 Summer（AtCoder Heuristic Contest 021）](https://atcoder.jp/contests/ahc021/submissions/42958555)を参考にした。

![eet](eet.jpg)

- ビームを深くするたびにEETを拡張する
- 四角で囲まれているのは、ビーム幅で選ばれた頂点
- 丸で囲まれているのは、木の葉
- 下線は、選ばれた頂点だけをEETで表現するために必要な辺

実装例 [number gather game](number_gather_game.cpp) [世界四連覇AIエンジニアがゼロから教えるゲーム木探索入門 - Qiita](https://qiita.com/thun-c/items/058743a25c37c87b8aa4)に載っている数集めゲームに対して適用した。