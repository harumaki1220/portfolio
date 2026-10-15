import { defineConfig } from 'astro/config';

export default defineConfig({
  site: 'https://harumaki1220.github.io',
  base: process.env.BASE_PATH ?? '/portfolio',
  markdown: {
    // コードのハイライト。ダーク / ライトの切り替えは global.css でやる
    shikiConfig: {
      themes: { light: 'github-light', dark: 'github-dark' },
    },
  },
  experimental: {
    // 記事のフロントマターを VS Code で補完・チェックできるようにする（.vscode/settings.json と合わせて使う）
    contentIntellisense: true,
  },
});
