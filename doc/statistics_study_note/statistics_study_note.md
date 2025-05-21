# 統計検定学習ノート

<link rel="stylesheet" type="text/css" href="https://tikzjax.com/v1/fonts.css">
<script src="https://tikzjax.com/v1/tikzjax.js"></script>

## TikZJax について

JavaScript で tikz を利用可能にするもの。すごい。参考：<https://tikzjax.com/>

以下のようにインクルードし、

```html
<link rel="stylesheet" type="text/css" href="https://tikzjax.com/v1/fonts.css">
<script src="https://tikzjax.com/v1/tikzjax.js"></script>
```

`<script type="text/tikz"></script>` タグの中に tikz を記述する。`html {cmd=true hide=true}` を指定しても、Markdown Preview Enhanced では実行してくれないようだ。

```html
<script type="text/tikz">
 \begin{tikzpicture}
 \draw (0,0) circle (1in);
 \end{tikzpicture}
</script>
```

<script type="text/tikz">
 \begin{tikzpicture}
 \draw (0,0) circle (1in);
 \end{tikzpicture}
</script>

## 指数と対数

対数はほぼ使わないけど。でも、たまに出てくるような…。

### 指数法則

指数に対して以下の法則が成り立つ。

$$
\newcommand{\arraystretch}{2.0}
\begin{align*}
& a \neq 0, b \neq 0 とし、m, n を整数とする \\
& (1) \space a^m a^n=a^{m+n} && (2) \space \cfrac{a^m}{a^n} = a^{m-n} \\
& (3) \space (a^m)^n = a^{m n} && (4) \space (ab)^n = a^n b^n \\
& (5) \space a^0 = 1 && (6) \space a^{-1} = \cfrac{1}{a} \\
& (7) \space a^{-n} = \cfrac{1}{a^n}
\end{align*}
$$

### 対数の定義

対数の定義は以下の通り。

$$
\begin{align*}
& a > 0, a \neq 1, M > 0 のとき \\
\\
& a^p = M \Leftrightarrow \log_a{M} = p \\
\\
& (1) \space \log_a{M} を a を底とする M の対数という。\\
& (2) \space M を \log_a{M} の真数という。
\end{align*}
$$

### 対数法則

対数に対して以下の法則が成り立つ。

$$
\newcommand{\arraystretch}{2.0}
\begin{align*}
& (1) \space \log_a{M} + \log_a{N} = \log_a{MN} \\
& (2) \space \log_a{M^p} = p\log_a{M} \\
& (3) \space \log_a{\cfrac{1}{M}} = - \log_a{M} \\
& (4) \space \log_a{M} - \log_a{N} = \log_a{\cfrac{M}{N}} \\
& (5) \space \log_a{1} = 0 \\
& (6) \space \log_a{b} = \cfrac{\log_c{b}}{\log_c{a}}
\end{align*}
$$

## 計算方法

### 平方完成

統計検定 3 級に登場した。出題されても 1 問程度なので捨ててよいかも。平方完成は $3x^2 -12x +6$ といった式の $x$ を $(x - n)^2$ の形にまとめる手法のこと。平方完成を使うと、$(x - n)^2 = X^2$ のように値を置き換えて考えることができるようになる。

$$
\newcommand{\arraystretch}{2.0}
\begin{align*}
&3x^2  - 12x + 6 \\
& = 3 (x^2 - 4x) + 6 & \dots & \text{ $x^2$ の係数を外に出す } \\
& = 3 (x^2 -2 \cdot 2x + 2^2 - 2^2) + 6 & \dots & \text{ $-4x$ の係数を $\cfrac{1}{2}$ にし、かつ $\cfrac{1}{2}$ にした値の2乗を足し引きする}\\
& = 3 \{(x - 2)^2 - 4\} + 6 & \dots & \text{ $x^2 -2 \cdot 2x + 2^2$ を因数分解する }\\
& = 3 (x - 2)^2 - 12 + 6 & \dots & \text{ $3$ を展開する } \\
& = 3 (x - 2)^2 - 6 & \dots & \text{ 数値を計算しておしまい }
\end{align*}
$$
