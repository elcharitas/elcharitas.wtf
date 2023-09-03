export const publicationsQuery = `
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
        }
      }
    }
  }
`;

export type PublicationsQueryResponse = {
  user: {
    publication: {
      posts: {
        title: string;
        dateAdded: string;
        coverImage: string;
        slug: string;
        brief: string;
        views: number;
        tags: {
          name: string;
        }[];
      }[];
    };
  };
};

export function transformBlog(
  post: PublicationsQueryResponse["user"]["publication"]["posts"][number]
): Post {
  return {
    title: post.title,
    date: post.dateAdded,
    brief: post.brief,
    coverImage: post.coverImage,
    slug: post.slug,
    views: post.views,
    type: "blog",
  };
}

export async function getAllBlogs() {
  const response = await fetch("https://api.hashnode.com", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      query: publicationsQuery.replace("$page", "0"),
    }),
  });

  const { data } = (await response.json()) as {
    data: PublicationsQueryResponse;
  };

  return data?.user?.publication?.posts.map(transformBlog) || [];
}
