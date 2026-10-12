import { defineCollection } from "astro:content";
import { glob } from "astro/loaders";
import { z } from "astro/zod";

const blog = defineCollection({
  // src/content/blog の中の .md ファイルを全部読み込む。ファイル名が URL になる
  loader: glob({ pattern: "**/*.md", base: "./src/content/blog" }),
  // 記事の先頭（フロントマター）に書く情報。足りないとビルドでエラーになる
  schema: z.object({
    title: z.string(),
    description: z.string(),
    date: z.coerce.date(),
    draft: z.boolean().default(false),
  }),
});

export const collections = { blog };
