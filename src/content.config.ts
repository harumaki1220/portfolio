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
    date: z.coerce.date(),
    draft: z.boolean().default(false),
    // 書かなければタグなし。使えるタグは src/data/tags.ts の一覧から
    tags: z.array(z.enum(TAGS)).default([]),
  }),
});

export const collections = { blog };
