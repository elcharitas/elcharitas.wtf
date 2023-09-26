import { kv } from "@vercel/kv";

export const userDataQuery = `
  query {
    user(username: "elcharitas") {
      publication {
        posts(page: $page) {
          title
          dateAdded
          coverImage
          slug
          brief
          views
          readTime
        }
      }
    }
  }
`;

export const blogPostQuery = `
  query {
    post(slug: $slug, hostname: "iamelcharitas.hashnode.dev") {
      title
      dateAdded
      coverImage
      contentMarkdown
      slug
      brief
      views
      readTime
    }
  }
`;

export type UserQueryResponse = {
  user: {
    publication: {
      posts: PostQueryResponse["post"][];
    };
  };
};

export type PostQueryResponse = {
  post: {
    title: string;
    dateAdded: string;
    coverImage: string;
    contentMarkdown?: string;
    slug: string;
    brief: string;
    views: number;
    readTime: number;
    tags: {
      name: string;
    }[];
  };
};

export async function transformBlog(
  post: PostQueryResponse["post"]
): Promise<Post> {
  const { views } =
    "location" in globalThis
      ? await (await fetch(`/blog/views?slug=${post.slug}`)).json()
      : { views: await kv.get<number>(`${post.slug}-views`) };
  return {
    title: post.title,
    date: post.dateAdded,
    brief: post.brief,
    coverImage: post.coverImage,
    slug: post.slug,
    views: post.views + views,
    content: post.contentMarkdown,
    type: "blog",
  };
}

export async function getAllBlogs(page = 0): Promise<Post[]> {
  const response = await fetch("https://api.hashnode.com", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      query: userDataQuery.replace("$page", page.toString()),
    }),
  });

  const { data } = (await response.json()) as {
    data: UserQueryResponse;
  };

  return await Promise.all(
    data?.user?.publication?.posts.map(transformBlog) || []
  );
}

export async function getBlogPost(slug: string): Promise<Post> {
  const response = await fetch("https://api.hashnode.com", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      query: blogPostQuery.replace("$slug", JSON.stringify(slug)),
    }),
  });

  const { data } = (await response.json()) as {
    data: PostQueryResponse;
  };

  return await transformBlog(data.post);
}
