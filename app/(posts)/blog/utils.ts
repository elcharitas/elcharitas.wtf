import { kv } from "@vercel/kv";
import type {
  PostsByPublicationQuery,
  SinglePostByPublicationQuery,
} from "@/graphql/graphql";
import PostsByPublication from "@/graphql/queries/PostsByPublication.graphql";
import SinglePostByPublication from "@/graphql/queries/SinglePostByPublication.graphql";
import { executeQuery } from "@/graphql/utils";

const viewCount = async (slug: string) => {
  return ("location" in globalThis
    ? await (await fetch(`/blog/views?slug=${slug}`)).json()
    : { views: await kv.get<number>(`${slug}-views`) }
  ).views;
};

export async function getAllBlogs(
  cursor: string | null = null
): Promise<[Post[], string | null]> {
  const { data } = await executeQuery<PostsByPublicationQuery>(
    { PostsByPublication },
    {
      host: "elcharitas.wtf/blog",
      first: 9,
      after: cursor,
    }
  );

  const { edges = [], pageInfo } = data?.publication?.posts ?? {};

  return [
    await Promise.all(
      edges.map(async ({ node: post }) => ({
        title: post.title,
        date: post.publishedAt,
        brief: post.brief,
        coverImage: post.coverImage?.url,
        slug: post.slug,
        views: post.views + (await viewCount(post.slug)),
        type: "blog",
      }))
    ),
    pageInfo?.hasNextPage && pageInfo.endCursor ? pageInfo.endCursor : null,
  ];
}

export async function getBlogPost(slug: string): Promise<Post> {
  const { data } = await executeQuery<SinglePostByPublicationQuery>(
    { SinglePostByPublication },
    {
      host: "elcharitas.wtf/blog",
      slug,
    }
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
    views: post.views + (await viewCount(post.slug)),
    content: post.content.markdown,
    type: "blog",
  };
}
