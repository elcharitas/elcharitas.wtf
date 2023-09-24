import Link from "next/link";
import React from "react";
import { ArrowDownCircle, Settings, Eye, Star } from "lucide-react";
import { Card } from "../components/card";
import { Article } from "./article";
import { Content } from "../components/content";

interface PostsPageProps {
  title: string;
  description?: string;
  featured?: Post[];
  sorted: Post[];
  isLoading?: boolean;
  isReachedEnd?: boolean;
  handleLoadMore?: () => void;
}

export function PostsPage({
  title,
  description,
  sorted,
  featured: [featured, ...others] = [],
  isLoading,
  isReachedEnd,
  handleLoadMore,
}: PostsPageProps) {
  return (
    <>
      <div className="max-w-2xl mx-auto lg:mx-0">
        <h2 className="text-3xl font-bold tracking-tight text-zinc-100 sm:text-4xl">
          {title}
        </h2>
        <p className="mt-4 text-zinc-400">
          <Content text={description} />
        </p>
      </div>
      <div className="w-full h-px bg-zinc-800" />

      {featured && (
        <div className="grid grid-cols-1 gap-8 mx-auto lg:grid-cols-2 items-center">
          <Card>
            <Link href={`/${featured.type}/${featured.slug}`}>
              <article className="relative w-full h-full p-4 md:p-8">
                <div className="flex items-center justify-between gap-2">
                  <div className="text-xs text-zinc-100">
                    {featured.date ? (
                      <time dateTime={new Date(featured.date).toISOString()}>
                        {Intl.DateTimeFormat("en-US", {
                          dateStyle: "medium",
                        }).format(new Date(featured.date))}
                      </time>
                    ) : (
                      <span>SOON</span>
                    )}
                  </div>
                  <span className="flex items-center gap-1 text-xs text-zinc-500">
                    {featured.type === "projects" ? (
                      <Star className="w-4 h-4" />
                    ) : (
                      <Eye className="w-4 h-4" />
                    )}{" "}
                    {Intl.NumberFormat("en-US", { notation: "compact" }).format(
                      featured.views ?? 0
                    )}
                  </span>
                </div>

                <div className="rounded-lg overflow-hidden w-full h-48 mx-auto mt-2">
                  <img
                    src={featured.coverImage}
                    alt={featured.title}
                    className="object-cover w-full h-full"
                  />
                </div>

                <h2
                  id="featured-post"
                  className="mt-4 text-3xl font-bold text-zinc-100 group-hover:text-white sm:text-4xl font-display"
                >
                  {featured.title}
                </h2>
                <p className="mt-4 leading-8 duration-150 text-zinc-400 group-hover:text-zinc-300 mb-8">
                  {featured.brief}
                </p>
                <div className="absolute bottom-4 md:bottom-8">
                  <p className="hidden text-zinc-200 hover:text-zinc-50 lg:block">
                    Read more <span aria-hidden="true">&rarr;</span>
                  </p>
                </div>
              </article>
            </Link>
          </Card>

          <div className="flex flex-col w-full gap-8 mx-auto border-t border-gray-900/10 lg:mx-0 lg:border-t-0 ">
            {others.map((post) => (
              <Card key={post.slug}>
                <Article post={post} showReadMore />
              </Card>
            ))}
          </div>
        </div>
      )}
      <div className="hidden w-full h-px md:block bg-zinc-800" />

      <div className="grid grid-cols-1 gap-4 mx-auto lg:mx-0 md:grid-cols-3">
        <div className="grid grid-cols-1 gap-4">
          {sorted
            .filter((_, i) => i % 3 === 0)
            .map((project) => (
              <Card key={project.slug}>
                <Article post={project} />
              </Card>
            ))}
        </div>
        <div className="grid grid-cols-1 gap-4">
          {sorted
            .filter((_, i) => i % 3 === 1)
            .map((project) => (
              <Card key={project.slug}>
                <Article post={project} />
              </Card>
            ))}
        </div>
        <div className="grid grid-cols-1 gap-4">
          {sorted
            .filter((_, i) => i % 3 === 2)
            .map((project) => (
              <Card key={project.slug}>
                <Article post={project} />
              </Card>
            ))}
        </div>
      </div>
      {handleLoadMore && !isReachedEnd && (
        <div className="flex justify-center items-center">
          <button
            disabled={isLoading}
            onClick={handleLoadMore}
            className={`flex justify-between items-center m-2 px-4 py-2 text-sm text-zinc-50  border border-zinc-700 rounded ${
              isLoading
                ? "bg-zinc-700 scale-110 rounded-xl"
                : "hover:bg-zinc-800 hover:scale-110 hover:rounded-xl duration-1000"
            }`}
          >
            {isLoading ? (
              <>
                Loading... <Settings className="w-4 h-4 m-2 animate-spin" />
              </>
            ) : (
              <>
                Load more <ArrowDownCircle className="w-4 h-4 m-2" />
              </>
            )}
          </button>
        </div>
      )}
    </>
  );
}
