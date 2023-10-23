import type {
  PostsByPublicationQuery,
  SinglePostByPublicationQuery,
  SearchPostsOfPublicationQuery,
} from "@/graphql/graphql";
import PostsByPublication from "@/graphql/queries/PostsByPublication.graphql";
import SinglePostByPublication from "@/graphql/queries/SinglePostByPublication.graphql";
import SearchPostsOfPublication from "@/graphql/queries/SearchPostsOfPublication.graphql";
import { executeQuery } from "@/graphql/utils";
import { first, publicationId, host } from "@/graphql/config";

export async function getAllBlogs(
  cursor: string | null = null
): Promise<[Post[], string | null]> {
  const { data } = await executeQuery<PostsByPublicationQuery>(
    { PostsByPublication },
    { host, first, after: cursor }
  );

  const { edges = [], pageInfo } = data?.publication?.posts ?? {};

  return [
    edges.map(({ node: post }) => ({
      title: post.title,
      date: post.publishedAt,
      brief: post.brief,
      coverImage: post.coverImage?.url,
      slug: post.slug,
      views: post.views,
      type: "blog",
    })),
    pageInfo?.hasNextPage && pageInfo.endCursor ? pageInfo.endCursor : null,
  ];
}

export async function searchBlogs(
  query: string,
  cursor: string | null = null
): Promise<[Post[], string | null]> {
  const { data } = await executeQuery<SearchPostsOfPublicationQuery>(
    { SearchPostsOfPublication },
    {
      host,
      filter: { publicationId, query },
      first,
      after: cursor,
    }
  );

  const { edges = [], pageInfo } = data?.searchPostsOfPublication ?? {};

  return [
    edges.map(({ node: post }) => ({
      title: post.title,
      date: post.publishedAt,
      brief: post.brief,
      coverImage: post.coverImage?.url,
      slug: post.slug,
      views: post.views,
      type: "blog",
    })),
    pageInfo?.hasNextPage && pageInfo.endCursor ? pageInfo.endCursor : null,
  ];
}

export async function getBlogPost(slug: string): Promise<Post> {
  const { data } = await executeQuery<SinglePostByPublicationQuery>(
    { SinglePostByPublication },
    { host, slug }
  );

  if (data?.publication?.post == null) {
    throw new Error("Post not found");
  }

  const { post } = data.publication;

  return {
    title: post.title,
    date: post.publishedAt,
    brief: post.brief,
    coverImage: post.coverImage?.url,
    slug: post.slug,
    views: post.views,
    content: post.content.markdown,
    type: "blog",
  };
}
