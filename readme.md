# niuristic

niuezがヒューリスティックについて勉強したメモをまとめておく場所 内容が正確かどうかは保証できません

## アルゴリズムについてのメモ

- 山登り・焼きなまし
    - 山登り・焼きなましの基礎
    - [遷移高速化・償却テク](./algorithm/simulated_annealing/sa_speedup.md)
        - 遷移貪欲化
        - Deque State
    - [非実行可能解をペナルティにするやつ](./algorithm/simulated_annealing/sa_infeasible_penalty.md)
    - 部分破壊+再構築
    - [多様性](./algorithm/simulated_annealing/sa_variety.md)
- ビームサーチ
    - ビームサーチの基礎
    - [高速化](./algorithm/beam_search/beam_search_speedup.md)
    - [評価関数](./algorithm/beam_search/beam_search_evaluation.md)
    - ハッシュの設計
- 推定
    - 注意!!!! 中の人は一回も推定回で勝てていません
    - [ギブスサンプリング](./algorithm/gibbs_sampling)
    - [ガウス分布を用いた推定・カルマンフィルタ](./algorithm/gaussian_distribution)
    - [ガウス回帰過程](./algorithm/gaussian_process)
- その他
    - [Tree Structed Parzen Estimator](./algorithm/tree_structed_parzen_estimator)(optunaが使用しているブラックボックス最適化アルゴリズム)
    - [Smooth Maximum](./algorithm/smooth_maximum)

## runner

[runner](./runner)

自分が使っている並列実行プログラム。`runner <IN_FOLDER> <OUT_FOLDER> [PROGRAM]...`と実行すると、`<input_filename>\\t<PROGRAMが出力したstderrの最終行>\\n`を出力してくれるので、それをスプレッドシートに貼り付けて観察している。

## ライブラリ

[utility](./utility)にまとめてある

## visualizer

[自作visualizer](https://github.com/niuez/egui_visualizer) 図形データの構造体とそれを格納・デコード・エンコードを行うフレーム構造体だけをクレートにし、そのクレートを、AHCの解法プログラムと、[描画を担当するeguiのアプリケーション](https://blog.niuez.net/egui_visualizer/)両方からインポートすることで簡素に作ってある。


## 復習

[practice](./practice)に復習の様子を残している
