import Link from "next/link";
import {
  Github,
  Mail,
  ExternalLink,
  Terminal,
  Code2,
  Globe,
  LayoutTemplate,
  Cpu,
} from "lucide-react";

export default function Home() {
  return (
    <div className="min-h-screen bg-slate-50 text-slate-900 font-sans">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-white/80 backdrop-blur-md border-b border-slate-200">
        <div className="container mx-auto px-4 h-16 flex items-center justify-between">
          <h1 className="text-xl font-bold tracking-tight">Haruma Kusunoki</h1>
          <nav className="flex gap-4 text-sm font-medium text-slate-600">
            <Link href="#skills" className="hover:text-blue-600 transition">
              Skills
            </Link>
            <Link href="#projects" className="hover:text-blue-600 transition">
              Projects
            </Link>
            <Link href="#contact" className="hover:text-blue-600 transition">
              Contact
            </Link>
          </nav>
        </div>
      </header>

      <main>
        {/* Hero Section */}
        <section className="py-20 md:py-32 container mx-auto px-4 text-center">
          <div className="text-xl md:text-2xl text-slate-700 font-medium mb-6">
            Haruma Kusunoki
          </div>

          <p className="text-lg text-slate-600 max-w-2xl mx-auto mb-10">
            モダンなWeb技術を学び、ユーザー体験を重視した開発を目指しています。
            <br className="hidden md:block" />
            GitHubでは <b>matcha(harumaki1220)</b>
            として活動し、日々コードを書いています。
          </p>

          <div className="flex justify-center gap-4">
            <Link
              href="#contact"
              className="bg-blue-600 text-white px-6 py-3 rounded-lg font-medium hover:bg-blue-700 transition"
            >
              Contact Me
            </Link>
            <Link
              href="https://github.com/harumaki1220"
              target="_blank"
              className="border border-slate-300 bg-white px-6 py-3 rounded-lg font-medium hover:bg-slate-50 transition flex items-center gap-2"
            >
              <Github size={20} /> GitHub: harumaki1220
            </Link>
          </div>
        </section>

        {/* Skills Section */}
        <section id="skills" className="py-20 bg-white">
          <div className="container mx-auto px-4">
            <h2 className="text-3xl font-bold text-center mb-12">
              Technical Skills
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-6 max-w-4xl mx-auto">
              {[
                {
                  name: "TypeScript",
                  icon: <Code2 className="text-blue-600" />,
                },
                {
                  name: "JavaScript",
                  icon: <Code2 className="text-yellow-500" />,
                },
                {
                  name: "React / Next.js",
                  icon: <Globe className="text-cyan-500" />,
                },
                {
                  name: "HTML5 / CSS3",
                  icon: <LayoutTemplate className="text-orange-500" />,
                },
                {
                  name: "Tailwind CSS",
                  icon: <Terminal className="text-teal-500" />,
                },
                {
                  name: "Git / GitHub",
                  icon: <Github className="text-slate-700" />,
                },
                {
                  name: "VS Code",
                  icon: <Cpu className="text-blue-500" />,
                },
              ].map((skill) => (
                <div
                  key={skill.name}
                  className="flex flex-col items-center p-6 bg-slate-50 rounded-xl border border-slate-100 hover:shadow-md transition"
                >
                  <div className="mb-4 p-3 bg-white rounded-full shadow-sm">
                    {skill.icon}
                  </div>
                  <span className="font-semibold text-slate-700">
                    {skill.name}
                  </span>
                </div>
              ))}
            </div>
          </div>
        </section>

        {/* Projects Section */}
        <section id="projects" className="py-20 bg-slate-50">
          <div className="container mx-auto px-4">
            <h2 className="text-3xl font-bold text-center mb-12">Projects</h2>
            <div className="grid md:grid-cols-2 lg:grid-cols-2 gap-8 max-w-5xl mx-auto">
              {/* Project 1: Are you a robot? */}
              <div className="bg-white rounded-xl overflow-hidden border border-slate-200 shadow-sm hover:shadow-lg transition flex flex-col">
                <div className="h-40 bg-indigo-50 flex items-center justify-center text-6xl">
                  🤖
                </div>
                <div className="p-6 flex-1 flex flex-col">
                  <div className="flex justify-between items-start mb-2">
                    <h3 className="text-xl font-bold">Are you a robot?</h3>
                    <span className="text-[10px] bg-yellow-100 text-yellow-800 px-2 py-0.5 rounded-full border border-yellow-200 font-medium">
                      WIP
                    </span>
                  </div>
                  <p className="text-slate-600 mb-4 text-sm flex-1">
                    初めてのハッカソンで製作中のプロジェクト。
                    逆reCAPTCHA（自分がロボットであることを判別する）アプリをチームで開発しています。
                  </p>
                  <div className="flex gap-2 mb-4 flex-wrap">
                    <span className="text-xs bg-blue-50 text-blue-700 px-2 py-1 rounded">
                      TypeScript
                    </span>
                    <span className="text-xs bg-blue-50 text-blue-700 px-2 py-1 rounded">
                      React
                    </span>
                    <span className="text-xs bg-blue-50 text-blue-700 px-2 py-1 rounded">
                      Next.js
                    </span>
                    <span className="text-xs bg-blue-50 text-blue-700 px-2 py-1 rounded">
                      Tailwind CSS
                    </span>
                  </div>
                  <div className="flex gap-4 mt-auto">
                    <a
                      href="https://github.com/harumaki1220/Are-you-a-robot"
                      target="_blank"
                      className="flex items-center gap-1 text-sm font-medium text-slate-600 hover:underline"
                    >
                      <Github size={16} /> Source Code
                    </a>
                  </div>
                </div>
              </div>

              {/* Project 2: Hohokekyo Converter */}
              <div className="bg-white rounded-xl overflow-hidden border border-slate-200 shadow-sm hover:shadow-lg transition flex flex-col">
                <div className="h-40 bg-orange-50 flex items-center justify-center text-5xl">
                  🐦
                </div>
                <div className="p-6 flex-1 flex flex-col">
                  <h3 className="text-xl font-bold mb-2">
                    Reversible Hohokekyo Converter
                  </h3>
                  <p className="text-slate-600 mb-4 text-sm flex-1">
                    文字コードを3進数に変換し、「ホ・ケ・キョ」にマッピングする可逆変換ロジックを実装。
                  </p>
                  <div className="flex gap-2 mb-4 flex-wrap">
                    <span className="text-xs bg-blue-50 text-blue-700 px-2 py-1 rounded">
                      React (Vite)
                    </span>
                    <span className="text-xs bg-blue-50 text-blue-700 px-2 py-1 rounded">
                      TypeScript
                    </span>
                  </div>
                  <div className="flex gap-4 mt-auto">
                    <a
                      href="https://harumaki1220.github.io/hohokekyo"
                      target="_blank"
                      className="flex items-center gap-1 text-sm font-medium text-blue-600 hover:underline"
                    >
                      <ExternalLink size={16} /> Live Demo
                    </a>
                    <a
                      href="https://github.com/harumaki1220/hohokekyo"
                      target="_blank"
                      className="flex items-center gap-1 text-sm font-medium text-slate-600 hover:underline"
                    >
                      <Github size={16} /> Source Code
                    </a>
                  </div>
                </div>
              </div>

              {/* Project 3: othello */}
              <div className="bg-white rounded-xl overflow-hidden border border-slate-200 shadow-sm hover:shadow-lg transition flex flex-col">
                <div className="h-40 bg-green-700 flex items-center justify-center text-5xl">
                  ⚪⚫
                </div>
                <div className="p-6 flex-1 flex flex-col">
                  <h3 className="text-xl font-bold mb-2">Reversi Game</h3>
                  <p className="text-slate-600 mb-4 text-sm flex-1">
                    初めての開発。 TypeScriptサークルにて作成したオセロゲーム。
                    useStateを用いて盤面の状態管理をすることを学びました。
                  </p>
                  <div className="flex gap-2 mb-4 flex-wrap">
                    <span className="text-xs bg-blue-50 text-blue-700 px-2 py-1 rounded">
                      TypeScript
                    </span>
                    <span className="text-xs bg-blue-50 text-blue-700 px-2 py-1 rounded">
                      React
                    </span>
                  </div>
                  <div className="flex gap-4 mt-auto">
                    <a
                      href="https://harumaki1220.github.io/othello/"
                      target="_blank"
                      className="flex items-center gap-1 text-sm font-medium text-blue-600 hover:underline"
                    >
                      <ExternalLink size={16} /> Live Demo
                    </a>
                    <a
                      href="https://github.com/harumaki1220/othello.git"
                      target="_blank"
                      className="flex items-center gap-1 text-sm font-medium text-slate-600 hover:underline"
                    >
                      <Github size={16} /> Source Code
                    </a>
                  </div>
                </div>
              </div>

              {/* Project 4: Minesweeper */}
              <div className="bg-white rounded-xl overflow-hidden border border-slate-200 shadow-sm hover:shadow-lg transition flex flex-col">
                <div className="h-40 bg-gray-200 flex items-center justify-center text-5xl">
                  💣
                </div>
                <div className="p-6 flex-1 flex flex-col">
                  <h3 className="text-xl font-bold mb-2">Minesweeper</h3>
                  <p className="text-slate-600 mb-4 text-sm flex-1">
                    TypeScriptサークルにて作成したマインスイーパー。
                    再帰関数を用いて空白を一気に開く処理を学びました
                  </p>
                  <div className="flex gap-2 mb-4 flex-wrap">
                    <span className="text-xs bg-blue-50 text-blue-700 px-2 py-1 rounded">
                      TypeScript
                    </span>
                    <span className="text-xs bg-blue-50 text-blue-700 px-2 py-1 rounded">
                      React
                    </span>
                  </div>
                  <div className="flex gap-4 mt-auto">
                    <a
                      href="https://harumaki1220.github.io/minesweeper/"
                      target="_blank"
                      className="flex items-center gap-1 text-sm font-medium text-blue-600 hover:underline"
                    >
                      <ExternalLink size={16} /> Live Demo
                    </a>
                    <a
                      href="https://github.com/harumaki1220/minesweeper"
                      target="_blank"
                      className="flex items-center gap-1 text-sm font-medium text-slate-600 hover:underline"
                    >
                      <Github size={16} /> Source Code
                    </a>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Contact Section */}
        <section id="contact" className="py-20 bg-white text-center">
          <div className="container mx-auto px-4">
            <h2 className="text-3xl font-bold mb-8">Contact</h2>
            <div className="flex flex-col items-center gap-4">
              <a
                href="mailto:haruma181220@gmail.com"
                className="flex items-center gap-2 text-slate-600 hover:text-blue-600 transition"
              >
                <Mail size={20} /> haruma181220@gmail.com
              </a>
            </div>
          </div>
        </section>
      </main>

      <footer className="bg-slate-50 py-8 text-center text-slate-500 text-sm border-t border-slate-200">
        © {new Date().getFullYear()} Haruma Kusunoki. All rights reserved.
      </footer>
    </div>
  );
}
