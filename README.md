# portfolio

https://harumaki1220.github.io/portfolio/

自己紹介、作ったもの (Works)、ブログ (Blogs) をまとめたサイトです。

## 使っている技術

- [Astro](https://astro.build/)
- TypeScript
- pnpm

## 動かし方

```bash
pnpm install
pnpm dev
```

| コマンド | 内容 |
| --- | --- |
| `pnpm dev` | 開発サーバーを起動する |
| `pnpm build` | `dist/` にサイトを書き出す |
| `pnpm preview` | 書き出したサイトを手元で確認する |
| `pnpm lint:text` | README とブログの記事の文章をチェックする |
| `pnpm lint:text:fix` | 自動で直せるものを直す |

## ブログの記事

`src/content/blog/` に Markdown のファイルを置くと記事になります。ファイル名がそのまま URL になります (`hello-world.md` → `/portfolio/blogs/hello-world/`)。

```md
---
title: 記事のタイトル
description: 一覧に出る一言
date: 2026-10-14
draft: true
---

本文
```

`draft: true` の記事は `pnpm dev` のときだけ表示され、公開されるサイトには出ません。

書いたら `pnpm lint:text` で文章をチェックします。ルールは `.textlintrc.json` にあります。

## デプロイ

`main` にプッシュすると、GitHub Actions (`.github/workflows/deploy.yml`) がビルドして GitHub Pages に公開します。

## クレジット

アイコンの一部は [Lucide](https://lucide.dev/) を使っています。
