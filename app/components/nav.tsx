"use client";
import { ArrowLeft } from "lucide-react";
import Link from "next/link";
import React, { useEffect, useRef, useState } from "react";

const navigation = [
  { name: "Blog", href: "/blog" },
  { name: "Projects", href: "/projects" },
  { name: "Resume", href: "/mods/resume" },
  { name: "Talks", href: "/talks" },
];

export const Navigation: React.FC = () => {
  const ref = useRef<HTMLElement>(null);
  const [isIntersecting, setIntersecting] = useState(true);

  useEffect(() => {
    if (!ref.current) {
      return;
    }
    const observer = new IntersectionObserver(([entry]) =>
      setIntersecting(entry.isIntersecting)
    );

    observer.observe(ref.current);
    return () => observer.disconnect();
  }, []);

  return (
    <header ref={ref}>
      <div
        className={`fixed inset-x-0 top-0 z-50 backdrop-blur  duration-200 border-b  ${
          isIntersecting
            ? "bg-zinc-900/0 border-transparent"
            : "bg-zinc-900/500  border-zinc-800 "
        }`}
      >
        <div className="container flex flex-row-reverse items-center justify-between p-6 mx-auto">
          <div className="flex justify-between gap-8">
            {navigation.map((item) => (
              <Link
                key={item.href}
                href={item.href}
                className="duration-200 text-zinc-400 hover:text-zinc-100"
              >
                {item.name}
              </Link>
            ))}
          </div>

          <Link
            href="/"
            className="text-zinc-300 hover:text-zinc-100 flex items-center gap-4 font-display text-xl text-transparent duration-1000 bg-white text-edge-outline animate-title whitespace-nowrap bg-clip-text "
          >
            <ArrowLeft className="w-6 h-6 " /> <span>elch🔥rit🔥s</span>
          </Link>
        </div>
      </div>
    </header>
  );
};
