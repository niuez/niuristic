# 部分破壊+再構築

## 部分破壊+再構築でないと(ほぼ)確実に焼けない例

- AHC002
- AHC010

## 小さい近傍と大きい近傍どちらを選択するか？

例: AHC045

上位陣は部分破壊の方が多そう

- [wanuiさん](https://zenn.dev/gmeriaog/articles/ddb38cd5e9530b)
- [JiroTechさん](https://atcoder.jp/contests/ahc045/submissions/64609408)
- [公式解説](https://speakerdeck.com/shun_pi/ahc045-jie-shuo?slide=29)

自分は小さい近傍を選択したがダメ(42位)だった。
W=0を試したが上位ほど良い得点が得られていない。

- 状態: 使用する辺の集合
- 評価関数: サンプリングしたケースの平均値+グループサイズ違反
- 遷移: 辺を追加したあとに削除など

何を基準に近傍の大きさを決定するか？

- 大きい近傍のメリット
    - 公式解説「2点swapだとグループの移動に時間がかかる」
        - これを違反ペナルティでどうにかしようとしたのだが...
    - ある状態の持ち方とそれを入力として(ほぼ)最適値が得られるアルゴリズムが存在する場合、部分破壊+再構築が有効か疑う余地あり？
        - AHC045: 各グループの頂点集合を制約した時の解の(ほぼ)最適解がMSTによって容易に求まる。
