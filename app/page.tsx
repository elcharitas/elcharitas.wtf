import Link from "next/link";
import {
  Github,
  Linkedin,
  Twitter,
  ExternalLink,
  Instagram,
} from "lucide-react";

const navigation = [
  { name: "Blog", href: "/blog" },
  { name: "Projects", href: "/projects" },
  { name: "Resume", href: "/mods/resume", noLink: true },
  { name: "Adventures", href: "/adventures" },
];

export default function Home() {
  return (
    <div className="flex flex-col items-center justify-center w-screen h-screen overflow-hidden bg-gradient-to-tl from-black via-zinc-600/20 to-black">
      <nav className="my-16 animate-fade-in">
        <div className="flex items-center justify-center gap-4">
          {navigation.map((item) =>
            item.noLink ? (
              <a
                key={item.href}
                href={item.href}
                className="text-sm duration-500 text-zinc-500 hover:text-zinc-300"
              >
                {item.name}
              </a>
            ) : (
              <Link
                key={item.href}
                href={item.href}
                className="text-sm duration-500 text-zinc-500 hover:text-zinc-300"
              >
                {item.name}
              </Link>
            )
          )}
        </div>
      </nav>
      <h1 className="z-10 text-4xl text-zinc-100 duration-1000 cursor-default text-edge-outline animate-title font-display sm:text-6xl md:text-9xl whitespace-nowrap">
        elch🔥rit🔥s
      </h1>
      <div className="my-16 mx-4 text-center animate-fade-in">
        <h2 className="text-sm text-zinc-500 max-sm:w-[300px]">
          Hi, I&apos;m Jonathan, I build interactive crypto dashboards at{" "}
          <a
            target="_blank"
            href="https://alphaday.com"
            className="underline duration-500 hover:text-zinc-300"
          >
            Alphaday
          </a>
          <br />
          while working on{" "}
          <Link
            target="_blank"
            href="https://crates.io/crates/ngyn"
            className="underline duration-500 hover:text-zinc-300"
          >
            Ngyn
          </Link>{" "}
          and{" "}
          <Link
            href="/projects/chakra-ui-svelte"
            className="underline duration-500 hover:text-zinc-300"
          >
            Chakra UI (svelte)
          </Link>{" "}
          at night. 😇
        </h2>
        <div className="pt-8">
          <a
            href="/mods/connect"
            target="_blank"
            rel="noopener noreferrer"
            className="px-4 py-2 text-sm text-white bg-zinc-700 border border-zinc-700 rounded hover:bg-zinc-600 duration-500"
            title="Let's get your questions answered"
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
          title="I collaborate on github often"
          aria-label="Follow me on github"
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
          aria-label="Connect with me on linkedin"
        >
          <Linkedin
            size={24}
            className="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110"
          />
        </a>
        <a
          href="https://instagram.com/iamelcharitas"
          target="_blank"
          rel="noopener noreferrer"
          className="text-zinc-500 hover:text-zinc-300 duration-500"
          aria-label="Connect with me on instagram"
        >
          <Instagram
            size={24}
            className="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110"
          />
        </a>
        <a
          href="https://twitter.com/iamelcharitas"
          target="_blank"
          rel="noopener noreferrer"
          className="text-zinc-500 hover:text-zinc-300 duration-500"
          aria-label="Follow me on twitter"
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
          aria-label="My links"
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
