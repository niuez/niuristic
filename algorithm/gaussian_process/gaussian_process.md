# Gaussian Process Regression

ガウス過程回帰は、観測値の集合$\mathcal{D} = \{(\bm{x}_1, y_1), \cdots, (\bm{x}_N, y_N)\}$から計算されるカーネル行列$\bm{K}$を用いて
$$Y \sim \mathcal{N}(\bm{0}, \bm{K})$$
に従うと仮定した上で、新しい点$\bm{x^*}$について$y^*$を推定する手法である。このため、$y$は平均0にしておく必要がある。

新しい点を含めても上は成り立つので、
$$
\begin{pmatrix}
\bm{y} \\
y^*
\end{pmatrix}
\sim
\mathcal{N}(\bm{0},
\begin{pmatrix}
\bm{K} & \bm{k}_* \\
\bm{k}_*^\top & k_{**}
\end{pmatrix}
)
$$
から、
$$
p(y^* | \bm{x}^*, \mathcal{D})
= \mathcal{N}(\bm{k}_* \bm{K}^{-1} \bm{y}, k_{**} - \bm{k}_*^\top \bm{K}^{-1} \bm{k}_*)
$$
となって、平均と分散が推定できる。

## 計算量

RBFカーネルで構成されるカーネル行列は実正定値行列となるので、コレスキー分解が可能である。
コレスキー分解によって、$A^{-1}b$の計算が、前計算$\mathcal{O}(n^3)$、クエリ$\mathcal{O}(n^2)$でできるので、回帰モデルの構成が$\mathcal{O}(n^3)$、一点の回帰が$\mathcal{O}(n^2)$でできる。

半正定値性をもつカーネルの場合、LU分解や普通に逆行列を求める方法がある。いずれにせよ計算量は変化しない。

## カーネル

RBFカーネル+ノイズのサンプルが[gaussian_process_regression.hpp](./gaussian_process_regression.hpp)に書いてある。

AHC018では、カーネルを独自構成する方法が強力っぽい。カーネル関数によって、$\bm{x}$と$\bm{x'}$の位置関係による$y$の共分散が得られればよいので、それを事前計算しておく。$y＄の正規化も非常に有効である。(参考: yosssさんによる[AHC018 ガウス過程回帰を用いた解法](https://docs.google.com/presentation/d/1JEcyHLw8XrDqL4FHUGYIVQC63KSZ2eaHRjO0E2y1WeU/edit#slide=id.g21378930346_0_62) p.26, 27)