"use client";
import { useRef, useState } from "react";
import { useAsyncEffect } from "@react-hook/async";
import { PostsPage } from "../post-page";
import { getAllBlogs } from "./utils";

interface BlogProps {
  initialCursor: string | null;
  initialPosts: Post[];
}

const BlogListing = ({ initialPosts, initialCursor }: BlogProps) => {
  const [nextCursor, setNextCursor] = useState<string | null>(null);

  const blogPosts = useRef(initialPosts);
  const [featured, top1, top2, ...sorted] = blogPosts.current;

  const { value: nextPageCursor, status } = useAsyncEffect(async () => {
    if (nextCursor) {
      const [currentPosts, nextPageCursor] = await getAllBlogs(nextCursor);
      if (currentPosts.length > 0) {
        const newPosts = currentPosts.filter(
          (c) => !blogPosts.current.find((p) => p.slug === c.slug)
        );
        blogPosts.current = [...blogPosts.current, ...newPosts];
      }
      return nextPageCursor;
    }
    return initialCursor;
  }, [nextCursor, initialCursor]);

  return (
    <PostsPage
      title="Blogs ✍🏼"
      description="I write about my experiences and thoughts on how to do software development, productivity, and life."
      featured={[featured, top1, top2]}
      sorted={sorted}
      isReachedEnd={!nextPageCursor}
      isLoading={status === "loading"}
      handleLoadMore={() => {
        if (nextPageCursor) {
          setNextCursor(nextPageCursor);
        }
      }}
    />
  );
};

export default BlogListing;
