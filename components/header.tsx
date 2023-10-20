"use client";
import Link from "next/link";
import React, { useEffect, useRef, useState } from "react";
import { List, Eye, Star, Twitter, Github, ExternalLink } from "lucide-react";

type Props = {
  post: Post;
};
export const Header: React.FC<Props> = ({ post }) => {
  const ref = useRef<HTMLElement>(null);
  const [isIntersecting, setIntersecting] = useState(true);

  const links: { label: string; href: string; icon: any }[] = [];
  if (post.owner) {
    links.push({
      label: "Source Code",
      href: `https://github.com/${post.owner}/${post.slug}`,
      icon: Github,
    });
  }
  if (post.url) {
    links.push({
      label: "View Docs",
      href: post.url,
      icon: ExternalLink,
    });
  }
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
    <header
      ref={ref}
      className="relative isolate overflow-hidden bg-no-repeat bg-center bg-gradient-to-tl from-black via-zinc-900 to-black"
      style={{
        backgroundImage:
          post.coverImage &&
          `linear-gradient(rgba(43, 43, 50, 0.9), rgba(43, 43, 50, 0.9)), url(${post.coverImage})`,
      }}
    >
      <div
        className={`inset-x-0 top-0 z-50 backdrop-blur lg:backdrop-blur-none duration-200 border-b lg:bg-transparent ${
          isIntersecting
            ? "bg-zinc-900/0 border-transparent"
            : "bg-white/10  border-zinc-200 lg:border-transparent"
        }`}
      >
        <div className="container flex flex-row-reverse items-center justify-between p-6 mx-auto">
          <div className="flex justify-between gap-8">
            <span
              title="View counter for this page"
              className={`duration-200 hover:font-medium flex items-center gap-1 ${
                isIntersecting
                  ? " text-zinc-400 hover:text-zinc-100"
                  : "text-zinc-600 hover:text-zinc-900"
              } `}
            >
              {post.type === "projects" ? (
                <Star className="w-5 h-5 text-yellow-200/60" />
              ) : (
                <Eye className="w-5 h-5 text-yellow-200/60" />
              )}{" "}
              {Intl.NumberFormat("en-US", { notation: "compact" }).format(
                post.views ?? 0
              )}
            </span>
            <Link target="_blank" href="https://twitter.com/iamelcharitas">
              <Twitter
                className={`w-6 h-6 duration-200 hover:font-medium ${
                  isIntersecting
                    ? " text-zinc-400 hover:text-zinc-100"
                    : "text-zinc-600 hover:text-zinc-900"
                } `}
              />
            </Link>
          </div>

          <Link
            href={`/${post.type}`}
            className={`duration-200 hover:font-medium ${
              isIntersecting
                ? " text-zinc-400 hover:text-zinc-100"
                : "text-zinc-600 hover:text-zinc-900"
            } `}
          >
            <List className="w-6 h-6 " />
          </Link>
        </div>
      </div>
      <div className="container mx-auto relative isolate overflow-hidden  py-24 sm:py-32">
        <div className="mx-auto max-w-7xl px-6 lg:px-8 text-center flex flex-col items-center">
          <div className="mx-auto max-w-2xl lg:mx-0">
            <h1 className="text-4xl font-bold tracking-tight text-white sm:text-6xl font-display">
              {post.title}
            </h1>
            <p className="mt-6 text-lg leading-8 text-zinc-300">
              {post.brief.substring(0, 120)}...
            </p>
          </div>

          <div className="mx-auto mt-10 max-w-2xl lg:mx-0 lg:max-w-none">
            <div className="grid grid-cols-1 gap-y-6 gap-x-8 text-base font-semibold leading-7 text-white sm:grid-cols-2 md:flex lg:gap-x-10">
              {links.map((link) => (
                <Link
                  target="_blank"
                  key={link.label}
                  href={link.href}
                  className="flex justify-between items-center px-4 py-2 text-sm text-zinc-50  border border-zinc-600 rounded hover:scale-110 hover:bg-zinc-900 duration-1000"
                >
                  {link.label} <link.icon className="w-4 h-4 ml-2" />
                </Link>
              ))}
            </div>
          </div>
        </div>
      </div>
    </header>
  );
};
