"use client";
import { useMemo, useState } from "react";
import { GraphQLClient, ClientContext, useQuery } from "graphql-hooks";
import { PostsPage } from "../post-page";
import { userDataQuery, UserQueryResponse, transformBlog } from "./utils";

interface BlogProps {
  initialPosts: Post[];
}

const client = new GraphQLClient({
  url: "https://api.hashnode.com",
});

const updateData = (
  prevData: UserQueryResponse,
  newData: UserQueryResponse
) => {
  const posts = [
    ...prevData.user.publication.posts,
    ...newData.user.publication.posts,
  ];
  return {
    ...prevData,
    user: {
      ...prevData.user,
      publication: { ...prevData.user.publication, posts },
    },
  };
};

const BlogListing = ({ initialPosts }: BlogProps) => {
  const [currentPage, setCurrentPage] = useState(1);

  const { data } = useQuery<UserQueryResponse>(
    userDataQuery.replace("$page", currentPage.toString()),
    {
      updateData,
    }
  );

  const [featured, top1, top2, ...sorted]: Post[] = useMemo(
    () => [
      ...initialPosts,
      ...(data?.user.publication?.posts || []).map(transformBlog),
    ],
    [initialPosts, data]
  );

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
