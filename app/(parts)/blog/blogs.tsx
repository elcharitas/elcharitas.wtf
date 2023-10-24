"use client";
import { useMemo, useRef, useState } from "react";
import { useAsyncEffect } from "@react-hook/async";
import { PostsPage } from "@/components/post-page";
import { getAllBlogs, searchBlogs } from "./utils";

interface BlogProps {
  initialCursor: string | null;
  initialPosts: Post<number>[];
}

const BlogListing = ({ initialPosts, initialCursor }: BlogProps) => {
  const [nextCursor, setNextCursor] = useState<string | null>(null);
  const [search, setSearch] = useState<string | null>(null);

  const blogPosts = useRef(initialPosts);

  const [featured, sorted] = useMemo(() => {
    if (search) {
      return [[], blogPosts.current];
    }
    const featured = blogPosts.current.slice(0, 3);
    const sorted = blogPosts.current.slice(3);

    return [featured, sorted];
  }, [blogPosts.current, search]);

  const { value: nextPageCursor, status } = useAsyncEffect(async () => {
    let currentPosts: Post<number>[] = [];
    let nextPageCursor: string | null = null;

    if (search && search.length >= 2) {
      [currentPosts, nextPageCursor] = await searchBlogs(search, nextCursor);
    } else if (nextCursor) {
      [currentPosts, nextPageCursor] = await getAllBlogs(nextCursor);
    }

    if (currentPosts.length > 0) {
      const newPosts = currentPosts.filter(
        (c) => !blogPosts.current.find((p) => p.slug === c.slug)
      );
      blogPosts.current = [...blogPosts.current, ...newPosts];
      return nextPageCursor;
    }

    return initialCursor;
  }, [nextCursor, initialCursor, search]);

  return (
    <PostsPage
      title="Blogs ✍🏼"
      description="I write about my experiences and thoughts on how to do software development, productivity, and life."
      featured={featured}
      sorted={sorted}
      isReachedEnd={status !== "loading" && !nextPageCursor}
      isLoading={status === "loading"}
      handleSearch={(value) => {
        blogPosts.current = [];
        if (!value) {
          blogPosts.current = initialPosts;
          setNextCursor(initialCursor);
        }
        setSearch(value);
      }}
      handleLoadMore={() => {
        if (nextPageCursor) {
          setNextCursor(nextPageCursor);
        }
      }}
    />
  );
};

export default BlogListing;
