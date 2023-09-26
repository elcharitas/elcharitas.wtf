"use client";
import { useRef, useState } from "react";
import { useAsyncEffect } from "@react-hook/async";
import { PostsPage } from "../post-page";
import { getAllBlogs } from "./utils";

interface BlogProps {
  initialPosts: Post[];
}

const BlogListing = ({ initialPosts }: BlogProps) => {
  const [currentPage, setCurrentPage] = useState(1);

  const blogPosts = useRef(initialPosts);
  const [featured, top1, top2, ...sorted] = blogPosts.current;

  const { value: hasReachedEnd, status } = useAsyncEffect(async () => {
    const currentPosts = await getAllBlogs(currentPage);
    if (currentPosts.length > 0) {
      const newPosts = currentPosts.filter(
        (c) => !blogPosts.current.find((p) => p.slug === c.slug)
      );
      blogPosts.current = [...blogPosts.current, ...newPosts];
      return (await getAllBlogs(currentPage + 1)).length === 0;
    }
    return currentPosts.length === 0;
  }, [currentPage]);

  return (
    <PostsPage
      title="Blogs ✍🏼"
      description="I write about my experiences and thoughts on how to do software development, productivity, and life."
      featured={[featured, top1, top2]}
      sorted={sorted}
      isReachedEnd={hasReachedEnd}
      isLoading={status === "loading"}
      handleLoadMore={() => {
        setCurrentPage((prev) => prev + 1);
      }}
    />
  );
};

export default BlogListing;
