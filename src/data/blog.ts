import { getCollection } from 'astro:content';

/**
 * 公開する記事を新しい順で返す。
 * draft: true の記事は、pnpm dev のときだけ含める（書きながら見た目を確認するため）
 */
export async function getPosts() {
  const posts = await getCollection(
    'blog',
    ({ data }) => import.meta.env.DEV || !data.draft,
  );
  return posts.sort((a, b) => b.data.date.getTime() - a.data.date.getTime());
}

/** 2026-10-12 → "2026.10.12" */
export function formatDate(date: Date): string {
  const y = date.getFullYear();
  const m = String(date.getMonth() + 1).padStart(2, '0');
  const d = String(date.getDate()).padStart(2, '0');
  return `${y}.${m}.${d}`;
}
