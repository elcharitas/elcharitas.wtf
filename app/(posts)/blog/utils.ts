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

export async function getAllBlogs(page = 0): Promise<Post[]> {
  const { data } = await executeQuery<PostsByPublicationQuery>(
    { PostsByPublication },
    {
      host: "elcharitas.wtf/blog",
      first: 12 * (page + 1),
    }
  );

  const { edges = [] } = data?.publication?.posts ?? {};

  return Promise.all(
    edges.map(async ({ node: post }) => ({
      title: post.title,
      date: post.publishedAt,
      brief: post.brief,
      coverImage: post.coverImage?.url,
      slug: post.slug,
      views: post.views + (await viewCount(post.slug)),
      type: "blog",
    }))
  );
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
