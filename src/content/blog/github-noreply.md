---
title: GitHub の noreply メールアドレス
description: GitHub のコミットに載るメールアドレスを隠す方法。
date: 2026-10-15
draft: false
tags: [GitHub]
---

## きっかけ

Claude を使って開発していたところ、コミットのときに noreply のメールアドレスにするかどうかを聞かれて、はじめて知りました。いい機会なので設定してみます。

## コミットにはメールアドレスが記録される

Git のコミットには、作者の名前とメールアドレスが記録されます。使われるのは、`git config` の `user.email` に設定したアドレスです。

公開リポジトリなら、このアドレスは誰でも見られます。GitHub のコミットの画面や、リポジトリを clone して `git log` を見るだけでわかります。

```bash
git log --format='%an <%ae>'
```

## noreply のメールアドレスとは

GitHub が用意している、公開しても困らない代わりのアドレスです。形は次のようになっています。

```text
数字のID+ユーザー名@users.noreply.github.com
```

このアドレスでコミットしても、GitHub 上では自分のコミットとして扱われます。プロフィールの草もちゃんと生えます。

## 設定の方法

### GitHub の設定

1. GitHub の Settings → Emails を開く
2. 「Keep my email addresses private」をオンにする
3. 表示された noreply のアドレスをコピーする

同じ画面には「Block command line pushes that expose my email」という設定もあります。オンにすると、本物のアドレスのままプッシュしようとしたときに、GitHub が止めてくれます。必要に応じてオンにしてください。

### 手元の Git の設定

コピーした noreply のアドレスを設定します。

```bash
git config --global user.email "数字のID+ユーザー名@users.noreply.github.com"
```

設定できたか確認します。

```bash
git config user.email
```

## 過去のコミットについて

過去のコミットのアドレスは、設定を変えても消えません。履歴を書き換えれば消せますが、コミットの ID が全部変わるうえ、`force push` しなければならずキケンなため、これから気をつけることにしました。

## おわりに

少しでも気になるのであれば設定しておくのがオススメです。
