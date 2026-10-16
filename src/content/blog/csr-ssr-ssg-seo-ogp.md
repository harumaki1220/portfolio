---
title: CSR と SSR と SSG と SEO と OGP
description: Web ページの作り方の違いと、検索や SNS での見え方の関係をまとめて、自分のサイトでも試してみました。
date: 2026-10-17
draft: false
tags: [CSR, SSR, SSG, SEO, OGP]
---

## はじめに

ポートフォリオの点数を測って改善してみたくなったので、やってみました。その前に、点数に関係する CSR・SSR・SSG・SEO・OGP についても整理しておきます。意味はなんとなく知っていましたが、それぞれの関係まで一度まとめておきたかったからです。

## ページの中身をどこで作るか

Web ページの HTML をどこで、いつ作るかで大きく 3 つに分かれます。

### CSR

CSR (クライアントサイドレンダリング) は、ブラウザで HTML を作る方法です。

1. サーバーは、中身がほとんど空っぽの HTML と JavaScript を返す
2. ブラウザが JavaScript を実行して、ページの中身を組み立てる

サーバーから届いた時点の HTML は、次のようにほぼ空っぽです。

```html
<body>
  <div id="app"></div>
  <script src="/main.js"></script>
</body>
```

ブラウザで開いて `Ctrl + U` (ページのソースを表示) を押すと、サーバーから届いた HTML を確認できます。

### SSR

SSR (サーバーサイドレンダリング) は、アクセスされるたびにサーバーで HTML を作る方法です。最初から中身の入った HTML が届くので、ブラウザはすぐに表示できます。そのかわり、HTML を作るためのサーバーを動かし続ける必要があります。

### SSG

SSG (静的サイト生成) は、ビルドのときに全ページの HTML を作っておく方法です。サーバーは、できあがったファイルを返すだけです。

中身が毎回変わらないブログやドキュメントに向いています。このサイトも、Astro を使って SSG で作っています。

### 比べてみる

|                       | CSR                  | SSR                          | SSG                    |
| --------------------- | -------------------- | ---------------------------- | ---------------------- |
| HTML を作る場所       | ブラウザ             | サーバー                     | 自分の PC や CI        |
| HTML を作るタイミング | 開いたあと           | アクセスのたび               | ビルドのとき           |
| 最初に届く HTML       | ほぼ空っぽ           | 中身あり                     | 中身あり               |
| サーバー              | ファイルを返すだけ   | 処理が必要                   | ファイルを返すだけ     |
| 向いているもの        | 管理画面などのアプリ | ログインで中身が変わるページ | ブログ、ポートフォリオ |

## SEO

SEO (検索エンジン最適化) は、Google などの検索結果に、ページがきちんと出るようにすることです。

検索エンジンは、クローラーというプログラムでページの中身を読みに来ます。CSR のページだと、クローラーが最初に受け取るのは、ほぼ空っぽの HTML です。Google のクローラーは JavaScript を実行できますが、後回しにされることもあります。そのため、CSR は SEO に弱いと言われます。

検索結果に出る説明文は、`<head>` の中にメタディスクリプションとして書きます。

```html
<meta name="description" content="ページの説明" />
```

## OGP

OGP (Open Graph Protocol) は、SNS に URL を貼ったときに出るカードの情報です。タイトル、説明、画像などを `<head>` に書いておきます。

```html
<meta property="og:title" content="ページのタイトル" />
<meta property="og:description" content="ページの説明" />
<meta property="og:image" content="https://example.com/image.png" />
```

SNS のクローラーは JavaScript を実行しません。そのため、CSR で後から `<head>` を書き換えても、カードには反映されません。OGP を使うなら、SSR か SSG で最初から HTML に書いておく必要があります。

## 自分のサイトで測ってみた

測ったのはトップページです。

### 改善前 (2026/10/16)

<!-- textlint-disable prh -->

| 項目           | 携帯電話 | デスクトップ |
| -------------- | -------- | ------------ |
| パフォーマンス | 91       | 99           |
| ユーザー補助   | 96       | 96           |
| おすすめの方法 | 100      | 100          |
| SEO            | 91       | 91           |

<!-- textlint-enable prh -->

- [携帯電話の結果](https://pagespeed.web.dev/analysis/https-harumaki1220-github-io-portfolio/syx5mvi21c?form_factor=mobile)
- [デスクトップの結果](https://pagespeed.web.dev/analysis/https-harumaki1220-github-io-portfolio/syx5mvi21c?form_factor=desktop)

### 指摘されたこと

- ユーザー補助：薄い抹茶色の背景に抹茶色の文字を置いたところのコントラストが足りない
- SEO：メタディスクリプションがない
- パフォーマンス：Google Fonts の読み込みで表示が止まる、アイコンの画像が重い

### やったこと

#### コントラストを直した

ライトモードの抹茶色を、ほんの少し濃くしました。薄い抹茶色の背景の上でも、読みやすさの基準 (4.5 : 1) を満たすようにしています。

#### メタディスクリプションと OGP を足した

全ページ共通のレイアウトで、`<head>` にメタディスクリプションと OGP を入れるようにしました。ブログの記事では、記事ごとの説明文を使っています。

```astro
<meta name="description" content={description} />
<meta property="og:title" content={title} />
<meta property="og:description" content={description} />
```

このサイトは SSG なので、ビルドの時点で HTML に書き込まれます。SNS のクローラーにもそのまま読まれます。

### 改善後 (2026/10/17)

<!-- textlint-disable prh -->

| 項目           | 携帯電話  | デスクトップ |
| -------------- | --------- | ------------ |
| パフォーマンス | 91 → 92   | 99 → 100     |
| ユーザー補助   | 96 → 100  | 96 → 100     |
| おすすめの方法 | 100 → 100 | 100 → 100    |
| SEO            | 91 → 100  | 91 → 100     |

<!-- textlint-enable prh -->

- [携帯電話の結果](https://pagespeed.web.dev/analysis/https-harumaki1220-github-io-portfolio/1gd5u9g12z?form_factor=mobile)
- [デスクトップの結果](https://pagespeed.web.dev/analysis/https-harumaki1220-github-io-portfolio/1gd5u9g12z?form_factor=desktop)

ユーザー補助と SEO が 100 になりました。携帯電話のパフォーマンスは、Google Fonts とアイコンの読み込みに手を付けていないので、ほぼ変わっていません。

このサイトは Astro を使った SSG なので、HTML の中身はクローラーに届いていました。SEO の点数が 91 だったのは、メタディスクリプションがなかったためです。

なお、PageSpeed Insights の SEO は、タイトルやメタディスクリプションなどの基本ができているかのチェックです。100 になったからといって、検索の順位が上がるわけではありません。

## おわりに

用語として知っていたことも、自分のサイトで測ってみると点数として目に見えるのがおもしろかったです。とくに SSG だと、メタディスクリプションや OGP を足すだけで、そのまま検索エンジンや SNS に届くのがよくわかりました。
