/**
 * ブログで使えるタグの一覧。
 * ここにないタグを記事に書くとビルドがエラーになるので、表記揺れ（Github と GitHub など）を防げる。
 * 新しいタグを使うときは、先にここに足す
 */
export const TAGS = [
  'Astro',
  'Git',
  'GitHub',
  'Markdown',
  'Rust',
  'TypeScript',
  'CSR',
  'SSR',
  'SSG',
  'SEO',
  'OGP',
] as const;

export type Tag = (typeof TAGS)[number];
