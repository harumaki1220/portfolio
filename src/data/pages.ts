/** ナビゲーションに並べるページ。path は base からの相対パス */
export const PAGES = [
  { label: "Home", path: "" },
  { label: "Works", path: "works/" },
  { label: "Blogs", path: "blogs/" },
];

/** base（/portfolio など）を付けた URL を作る */
export function url(path: string): string {
  return `${import.meta.env.BASE_URL.replace(/\/$/, "")}/${path}`;
}
