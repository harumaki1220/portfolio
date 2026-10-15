import { defineCollection } from 'astro:content';
import { glob } from 'astro/loaders';
import { z } from 'astro/zod';
import { TAGS } from './data/tags';

const blog = defineCollection({
  // src/content/blog の中の .md ファイルを全部読み込む。ファイル名が URL になる
  loader: glob({ pattern: '**/*.md', base: './src/content/blog' }),
  // 記事の先頭（フロントマター）に書く情報。足りないとビルドでエラーになる
  schema: z.object({
    title: z.string(),
    description: z.string(),
    // YAML では 2026-10-15 が Date として読まれるので、いったん "2026-10-15" の文字列に戻してからチェックする。
    // こうすると、エディタ側のスキーマも「日付（date）」になり、日時でなくても警告が出ない
    date: z
      .preprocess(
        (value) =>
          value instanceof Date ? value.toISOString().slice(0, 10) : value,
        z.iso.date(),
      )
      .transform((value) => new Date(value)),
    draft: z.boolean().default(false),
    // 書かなければタグなし。使えるタグは src/data/tags.ts の一覧から
    tags: z.array(z.enum(TAGS)).default([]),
  }),
});

export const collections = { blog };
