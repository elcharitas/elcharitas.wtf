"use client";
import { useState } from "react";
import { GraphQLClient, ClientContext, useQuery } from "graphql-hooks";
import { PostsPage } from "../post-page";
import {
  publicationsQuery,
  PublicationsQueryResponse,
  transformBlog,
} from "./utils";

interface BlogProps {
  initialPosts: Post[];
}

const client = new GraphQLClient({
  url: "https://api.hashnode.com",
});

const BlogListing = ({ initialPosts }: BlogProps) => {
  const [currentPage, setCurrentPage] = useState(1);
  const { data } = useQuery<PublicationsQueryResponse>(
    publicationsQuery.replace("$page", currentPage.toString())
  );

  const [featured, top1, top2, ...sorted]: Post[] = [
    ...initialPosts,
    ...(data?.user.publication?.posts || []).map(transformBlog),
  ];

  return (
    <PostsPage
      title="✍🏼 Blogs"
      description="I write about my experiences and thoughts on how to do software development, productivity, and life."
      featured={[featured, top1, top2]}
      sorted={sorted}
    />
  );
};

const Blogs = ({ initialPosts }: BlogProps) => {
  return (
    <ClientContext.Provider value={client}>
      <BlogListing initialPosts={initialPosts} />
    </ClientContext.Provider>
  );
};

export default Blogs;
