# Gaussian Distributionに関する知見集

---

# AHC003をガウス分布で推定

参考
1. [AHC003の2.926T解法+経緯 - Qiita](https://qiita.com/contramundum/items/b945400b81536df42d1a)
2. [雑記： 分散共分散行列のベイズ更新の話 - クッキーの日記](https://cookie-box.hatenablog.com/entry/2021/01/31/223006)
3. パターン認識と機械学習 上
    1. 2.3.1 条件付きガウス分布
    2. 2.3.3 ガウス変数におけるベイズの定理
    3. 2.3.4 ガウス分布の最尤推定

$M=1$について議論する。

$`D`$が与えられると仮定して、全ての辺の重みをベクトル$`x`$と表す。

## 辺の重みxの分布

xの平均$`\mu_{x0}`$と分散共分散行列$`\Sigma_{x0}`$は求めることができ、$`x \sim \mathcal{N}(\mu_{x0}, \Sigma{x0})`$を仮定する。

$`\mu_{x0} = \mathbb{E}[x]`$の各要素は$`(9000 - D + 1000 + D) / 2 = 5000`$
$`\Sigma_{x0}`$を考える。
- 同じ直線上ではない辺2つの間の共分散は0
- 同じ直線上である辺2つ$`x_i, x_j`$について
    - その直線上の重み$`H = \mathtt{rand}(1000+D, 9000-D)`$
    - 各辺の設定に使われる$`\delta_i = \mathtt{rand}(-D, D)`$
とすると、
```math
\begin{eqnarray}
&& \mathbb{E}[(H + \delta_i - 5000)(H + \delta_j - 5000)] \\
&=& \mathbb{E}[(H - 5000)^2] + \mathbb{E}[\delta_i \delta_j]
\end{eqnarray}
```
```math
\begin{eqnarray}
\mathbb{E}[(H - 5000)^2] &=& \int_{1000+D}^{9000-D} (x - 5000) \frac{1}{8000 - 2D} dx \\
&=& \frac{1}{3}(4000 - D)^2
\end{eqnarray}
```
$`i = j`$ならば、
```math
\begin{eqnarray}
\mathbb{E}[\delta_i \delta_j] &=& \int_{D}^{-D} x^2 \frac{1}{2D} dx \\
&=& \frac{1}{3}D^2
\end{eqnarray}
```
$`i \neq j`$ならば、$`\delta_i, \delta_j`$は独立なので$`\mathbb{E}[\delta_i \delta_j] = 0`$と計算できる。

## クエリ結果yの分布

$`y_t = b_t e_t`$を$`t`$回目のクエリの結果とする。クエリで用いた辺を1、それ以外を０としたベクトル$`c_t`$を用いれば、$`b_t = c_t x`$である。
これも、ガウス分布に従うと仮定して、$`y_t \sim \mathcal{N}(c_i x, R_t), R_t = \frac{1}{3}(0.1y_t)^2`$とする。

## 知りたいモデルとは？

最初持っているモデルは、$`p(x) = \mathcal{N}(x | \mu_{x0}, \Sigma_{x0})`$である。
これを用いて、1回目のクエリ$`p(y_1 | x) = \mathcal{N}(y_1 | c_i x, R_1)`$の分布が得られる。
これに対して、$`p(x | y_1)`$は、$y_1$の結果を踏まえたモデルとなる。ベイズの定理から、$`p(x | y_1) \propto p(y_1 | x) p(x)`$と求めることができる。
$`p(y_1 | x), p(x)`$が両方ガウス分布であるところから、$`p(x | y_1)`$もガウス分布に従う。これの平均と分散$`\mu_{x1}, \Sigma_{x1}`$を求めたい。

## $`p(x | y_1)`$のモデルの計算方法

二つの方法がある。参考2参照

- 同時分布`p(x, y_1)`を求めて計算する方法
    - 参考3.1
- 参考3.2を用いる方法
    - PRMLの通り求められる

今回は同時分布から求める。

同時分布のモーメント母関数を求める。$`y_1 = c_1^\top x + w_1`$, $`w_1 \sim \mathcal{N}(0, R_1)`$とする。

```math
\begin{eqnarray}
M_{x,y}(t)
&=& \mathcal{E}(e^{(t_x^\top t_y^\top)} \begin{pmatrix}x \\ y\end{pmatrix} ) \\
&=& \mathcal{E}(e^{(t_x + c_1 t_y)^\top x}) \mathcal{E}(e^{t_y^\top w}) \\
&=& \exp(\mu_{x0}^\top (t_x + c_1 t_y) + \frac{1}{2}+ (t_x + c_1 t_y)^\top \Sigma_{x0} (t_x + c_1 t_y)) \\
&& \cdot \ \exp(\frac{1}{2}t_y^\top R_t t_y) \\
&=& \exp((\mu_{x0}^\top \ \mu_{x0}^\top c_1) \begin{pmatrix}t_x \\ t_y\end{pmatrix} + \frac{1}{2} \begin{pmatrix}t_x \\ t_y\end{pmatrix}^\top \begin{pmatrix}\Sigma_{x0} & \Sigma_{x0} c_1 \\ c_1^\top \Sigma_{x0} & c_1^\top \Sigma_{x0} c_1 + R_1 \end{pmatrix} \begin{pmatrix}t_x \\ t_y\end{pmatrix})
\end{eqnarray}
```

参考3の式(2.81), (2.82)から、

```math
\mu_{x1} = \mu_{x|y_1} = \mu_{x0} + \Sigma_{x0}c_1 (c_1^\top \Sigma_{x0} c_1)^{-1} (y_1 - \mu_{x0}^\top c_1)  \\
\Sigma_{x1} = \Sigma_{x|y_1} = \Sigma_{x0} + (\Sigma_{x0} c_1) (c_1^\top \Sigma_{x0} c_1 + R_1)^{-1} (c_1^\top \Sigma_{x0})
```

これから、t回目のクエリのモデルについても逐次的に計算できる。

## モデルの確からしさ

対数尤度$`l_t := \log p(y_t | y_1, \dots, y_{t-1})`$を計算したい。上のモーメント母関数の式から、

```math
p(y_t | y_1, \cdots, y_{t-1}) \sim \mathcal{N}(\mu_{x0}^\top c_1, c_1^\top \Sigma_{x0} c_1 + R_1)
```
だから、参考3の式(2.118)より、

```math
l_t = - \frac{1}{2}(\log(2 \pi) + \log | c_1^\top \Sigma_{x0} c_1^\top + R_1 | + (y_t - \mu_{x0}^\top c_1)^\top (c_1^\top \Sigma_{x0} c_1^\top + R_1)^{-1} (y_t - \mu_{x0}^\top c_1)  )
```

これを用いて$`M, D`$のパラメータを推定できる。


---

# カルマンフィルタ

## 参考

1. [saharanさんのAHC040の提出](https://atcoder.jp/contests/ahc040/submissions/60610092)
2. [カルマンフィルターについて #機械学習 - Qiita](https://qiita.com/s-yonekura/items/7cdc99ba444cdbd4e880)
3. PRML
4. [微分エントロピー - Wikipedia](https://ja.wikipedia.org/wiki/%E5%BE%AE%E5%88%86%E3%82%A8%E3%83%B3%E3%83%88%E3%83%AD%E3%83%94%E3%83%BC)
5. [THIRD プログラミングコンテスト2023 (AHC030) 解説 - TERRYのブログ](https://www.terry-u16.net/entry/ahc030)

## カルマンフィルタ?導入

今回は参考2.において$`A = I`$のみを考えるため、カルマンフィルタというよりは[AHC003の例のやつ](./ahc003.md)とほぼ同じ。

$`X \sim N(x_0, S_0), Y | X \sim N(c X, R)`$とする。$`X | (Y = y)`$は$`N(x_1, S_1)`$に従う。ここで、

```math
\begin{align}
K &= \Sigma c^T ( c \Sigma c^T + R)^{-1} \\
x_1 &= x_0 + K (y - c x_0) \\
S_1 &= S_0 - K c \Sigma
\end{align}
```

である(PRML p.85 式(2.81) (2.82)にも同じようなことが書いてあるので、この方法でも導出可能そう)。

## (微分)エントロピーと相互情報量

確率変数$`X`$から一つ信号を受けた時の情報量の平均をエントロピー$`H(X)`$と呼ぶ。$`X`$が連続の場合は微分エントロピーと呼ぶ。微分エントロピーは負の値を取るなど、エントロピーと比較して望まない性質を持つが議論は割愛する。

$`X`$の確率密度が$`p(x)`$で表されるとき、微分エントロピー$`H(X)`$は以下のように定義される。

```math
H(X) := - \int p(x) \ln p(x) dx
```

さらに、$`X, Y`$の2つの確率変数があって、それぞれ確率密度が$`p(x), p(y)`$であるとする。$`Y`$の値が既知であるときに、$`X`$を特定するために必要な情報量を条件付きエントロピー$`H(X | Y)`$と呼び、以下のように定義される。

```math
H(X | Y) := - \int \int p(y) p(x|y) \ln p(x|y) dx dy
```

相互情報量$`I(X; Y) = H(X) - H(X | Y) = H(Y) - H(Y | X)`$は、$`Y`$を知ることによって、$`X`$の不確実性がどれだけ減少するかを表す(またその逆も言える)。

n変量ガウス分布$`X \sim N(m, \Sigma)`$の微分エントロピーは、$`H(X) = \frac{1}{2} (p \ln(2 \pi e) + \ln \det \Sigma)`$である。

## AHC040における相互情報量

横向きに大量に積んだクエリを投げ、カルマンフィルタによって推定することを考える。クエリを投げる際、相互情報量が大きくなるようなクエリを投げたい。つまり、$`H(Y) - H(Y | X)`$を最大にするようなクエリを投げたい。

$`H(Y | X)`$について、$`Y | X \sim N(cX, R)`$より、AHC040では$`R`$が定数であることから$`H(Y | X)`$は定数である。  
$`H(Y)`$について、$`Y \sim N(cX, c \Sigma c^T + R)`$であるため(導出はPRML p.90)、$`\det (c \Sigma c^T + R)`$が大きいほど、相互情報量$`I(X; Y)`$が大きくなる。

クエリの構成はsaharaさんは山登りによって行っている。$`c' = c + \Delta c`$と変化させるときの$`\det (c' \Sigma c'^T + R)`$を、

```math
c' \Sigma c'^T + R = c \Sigma c^T + 2 c \Sigma \Delta c + \Delta c \Sigma \Delta c + R
```

を使って差分計算を行っている。
