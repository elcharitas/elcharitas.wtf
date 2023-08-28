import Link from "next/link";
import React from "react";
import { Github, Linkedin, Twitter, ExternalLink } from "lucide-react";

const navigation = [
  { name: "Blog", href: "/blog" },
  { name: "Projects", href: "/projects" },
  { name: "Resume", href: "/resume" },
  { name: "Talks", href: "/talks" },
];

export default function Home() {
  return (
    <div className="flex flex-col items-center justify-center w-screen h-screen overflow-hidden bg-gradient-to-tl from-black via-zinc-600/20 to-black">
      <nav className="my-16 animate-fade-in">
        <ul className="flex items-center justify-center gap-4">
          {navigation.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              className="text-sm duration-500 text-zinc-500 hover:text-zinc-300"
            >
              {item.name}
            </Link>
          ))}
        </ul>
      </nav>
      <h1 className="z-10 text-4xl text-transparent duration-1000 bg-white cursor-default text-edge-outline animate-title font-display sm:text-6xl md:text-9xl whitespace-nowrap bg-clip-text ">
        elch🔥rit🔥s
      </h1>
      <div className="my-16 mx-4 text-center animate-fade-in">
        <h2 className="text-sm text-zinc-500 ">
          Hi, my name is Jonathan, I'm currently building amazing crypto
          dashboards at{" "}
          <Link
            target="_blank"
            href="https://alphaday.com"
            className="underline duration-500 hover:text-zinc-300"
          >
            Alphaday
          </Link>
          <br />
          and working on{" "}
          <Link
            target="_blank"
            href="/projects/rustle-rs"
            className="underline duration-500 hover:text-zinc-300"
          >
            Rustle.rs
          </Link>{" "}
          and{" "}
          <Link
            target="_blank"
            href="/projects/chakra-ui-svelte"
            className="underline duration-500 hover:text-zinc-300"
          >
            Chakra UI (svelte)
          </Link>{" "}
          at night.
        </h2>
        <div className="pt-8">
          <a
            href="mailto:your-email@example.com"
            className="px-4 py-2 text-sm text-white hover:text-black bg-zinc-700 border border-zinc-700 rounded hover:bg-zinc-300 duration-500"
          >
            Connect with me 🤝
          </a>
        </div>
      </div>
      <div className="flex justify-center space-x-4 my-4">
        <a
          href="https://github.com/elcharitas"
          target="_blank"
          rel="noopener noreferrer"
          className="text-zinc-500 hover:text-zinc-300 duration-500"
        >
          <Github
            size={24}
            className="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110"
          />
        </a>
        <a
          href="https://linkedin.com/in/elcharitas"
          target="_blank"
          rel="noopener noreferrer"
          className="text-zinc-500 hover:text-zinc-300 duration-500"
        >
          <Linkedin
            size={24}
            className="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110"
          />
        </a>
        <a
          href="https://twitter.com/iamelcharitas"
          target="_blank"
          rel="noopener noreferrer"
          className="text-zinc-500 hover:text-zinc-300 duration-500"
        >
          <Twitter
            size={24}
            className="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110"
          />
        </a>
        <a
          href="https://links.dev/elcharitas"
          target="_blank"
          rel="noopener noreferrer"
          className="text-zinc-500 hover:text-zinc-300 duration-500"
        >
          <ExternalLink
            size={24}
            className="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110"
          />
        </a>
      </div>
    </div>
  );
}
