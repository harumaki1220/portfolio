---
title: Markdown の書き方の見本
description: 記事の書き方と、実際の見え方をまとめた見本です。公開しません。
date: 2026-01-01
draft: true
tags: [Markdown]
---

この記事は書き方の見本です。`draft: true` なので、`pnpm dev` のときだけ表示されます。各項目は「書き方」のあとに「表示」を並べています。

## フロントマター

ファイルの先頭に書きます。

```md
---
title: 記事のタイトル
description: 一覧に出る一言
date: 2026-10-14
draft: true
tags: [GitHub, Git]
---
```

- `title` は記事の見出し、一覧、ブラウザのタブに出る
- `description` は一覧でタイトルの下に出る
- `draft: true` の間は公開されない
- `tags` は一覧と記事にタグとして出る（書かなくてもいい）
- `tags` で使えるのは `src/data/tags.ts` の一覧にあるタグだけ。新しいタグはまず一覧に足す

記事のタイトルは `title` が自動で出すので、本文に `# タイトル` は書きません。本文の見出しは `##` から始めます。

## 見出し

```md
## 見出し

### 小見出し
```

### 小見出し

これが小見出しの表示です。

## 段落と改行

1回改行しただけでは、前の行とつながります。段落を分けるときは空行を入れます。

```md
1行目です。
2行目です。

3行目です。
```

1行目です。
2行目です。

3行目です。

## 文字の装飾

```md
**太字** と `インラインコード`
```

**太字** と `インラインコード`

## リンク

```md
[GitHub](https://github.com/harumaki1220)
[作品一覧](/portfolio/works/)
```

[GitHub](https://github.com/harumaki1220)

[作品一覧](/portfolio/works/)

サイト内へのリンクは `/portfolio` から書きます。

## リスト

```md
- 項目
- 項目
  - 入れ子の項目

1. 1つ目
2. 2つ目
```

- 項目
- 項目
  - 入れ子の項目

1. 1つ目
2. 2つ目

## 引用

```md
> 引用した文
```

> 引用した文

## コードのブロック

最初の ` ``` ` のあとに言語名を書くと、色が付きます。

````md
```rust
fn main() {
    println!("Hello, World!");
}
```
````

```rust
fn main() {
    println!("Hello, World!");
}
```

```ts
const greeting: string = 'Hello, World!';
console.log(greeting);
```

## 表

```md
| 名前 | 内容         |
| ---- | ------------ |
| Rust | よく使う     |
| C    | 講義で使った |
```

| 名前 | 内容         |
| ---- | ------------ |
| Rust | よく使う     |
| C    | 講義で使った |

## 画像

記事と同じフォルダーに画像を置いて、相対パスで書きます。

```md
![画像の説明](./screenshot.png)
```

## 書いたあと

`pnpm lint:text` で文章をチェックします。自動で直せるものは `pnpm lint:text:fix` で直ります。
