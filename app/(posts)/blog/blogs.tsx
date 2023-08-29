import { useQuery } from "graphql-hooks";
import { PostsPage } from "../post-page";

const publicationsQuery = `
  query {
    user(username: "elcharitas") {
      publication {
        posts(page: 0) {
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

type PublicationsQueryResponse = {
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

const Blogs = () => {
  const { data } = useQuery<PublicationsQueryResponse>(publicationsQuery);
  const [featured, top1, top2, ...sorted]: Post[] =
    data?.user.publication?.posts.map((post) => ({
      title: post.title,
      date: post.dateAdded,
      url: "",
      brief: post.brief,
      coverImage: post.coverImage,
      slug: post.slug,
      type: "blog",
    })) || [];
  return (
    <PostsPage
      title="✍🏼 Blogs"
      description="I write about my experiences and thoughts on how to do software development, productivity, and life."
      featured={[featured, top1, top2]}
      sorted={sorted}
    />
  );
};

export default Blogs;
