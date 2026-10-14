export const NAME = 'まっちゃ';

export type SkillCategory = {
  name: string;
  main: string[];
  others: string[];
};

export const SKILLS: SkillCategory[] = [
  {
    name: 'Languages',
    main: ['Rust', 'TypeScript'],
    others: ['C', 'Python'],
  },
  {
    name: 'Frameworks / Libraries',
    main: ['React', 'Hono', 'Tailwind CSS', 'Zod'],
    others: ['Next.js'],
  },
  {
    name: 'Tools',
    main: [
      'Vite',
      'Git',
      'pnpm',
      'VS Code',
      'Docker',
      'PostgreSQL',
      'Linux (WSL2)',
    ],
    others: [],
  },
];

/** 自己紹介。1要素が1段落になる */
export const ABOUT = [
  '情報系の大学2年生です。普段は Web フロントエンドを書きつつ、CLI ツールやブラウザ拡張など、作りたくなったものを作っています。',
  '難しいと聞くと触りたくなる性格で、最近は Rust にハマっています。',
  '名前の由来は抹茶が好きだから🍵',
];

export const LIKES = [
  'プログラミング',
  '抹茶',
  'ラーメン',
  '音ゲー',
  '音楽（理芽、笹川真生、長谷川白紙）',
];

export type Work = {
  name: string;
  /** "2025.04" の形 */
  date: string;
  description: string;
  tech: string[];
  links: { label: string; href: string }[];
};

/** 新しい順に並べる */
export const WORKS: Work[] = [
  {
    name: 'portfolio',
    date: '2026.10',
    description: 'このサイト',
    tech: ['Astro', 'TypeScript'],
    links: [
      { label: 'Source', href: 'https://github.com/harumaki1220/portfolio' },
    ],
  },
  {
    name: 'minesweeper',
    date: '2025.05',
    description:
      'オセロの次に、TypeScript のサークルで作ったマインスイーパーです。再帰関数で空白のマスを一気に開く処理を学びました。',
    tech: ['TypeScript', 'React', 'Next.js'],
    links: [
      { label: 'Demo', href: 'https://harumaki1220.github.io/minesweeper/' },
      { label: 'Source', href: 'https://github.com/harumaki1220/minesweeper' },
    ],
  },
  {
    name: 'othello',
    date: '2025.04',
    description:
      '初めての開発。TypeScript のサークルで作ったオセロです。useState で盤面の状態を管理することを学びました。',
    tech: ['TypeScript', 'React', 'Next.js'],
    links: [
      { label: 'Demo', href: 'https://harumaki1220.github.io/othello/' },
      { label: 'Source', href: 'https://github.com/harumaki1220/othello' },
    ],
  },
];

export type Icon = 'github' | 'x';

export const LINKS: { icon: Icon; label: string; href: string }[] = [
  { icon: 'github', label: 'GitHub', href: 'https://github.com/harumaki1220' },
  { icon: 'x', label: 'X', href: 'https://x.com/matcha445_dev' },
];

/** GitHub のアイコンをそのまま表示する */
export const ICON_URL = 'https://github.com/harumaki1220.png?size=256';

/** アイコンを描いてくれた人 */
export const ICON_CREDIT = {
  label: '@yng_hoti',
  href: 'https://x.com/yng_hoti',
};
