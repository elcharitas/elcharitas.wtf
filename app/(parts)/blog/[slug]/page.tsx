import { notFound } from "next/navigation";
import { Mdx } from "@/components/mdx";
import { Header } from "@/components/header";
import { getBlogPost } from "../utils";

export const revalidate = 30; // 1 min

type Props = {
  params: {
    slug: string;
  };
};

export default async function BlogInfo({ params }: Props) {
  const blogPost = await getBlogPost(params.slug);

  if (!blogPost) {
    notFound();
  }

  return (
    <div className="bg-zinc-800 min-h-screen">
      <Header post={blogPost} />
      <article className="px-4 md:px-0 py-12 md:mx-auto prose prose-zinc prose-quoteless">
        <Mdx
          code={(blogPost.content ?? "").replace(
            /align=\"(left|right|center)\"/g,
            ""
          )}
        />
      </article>
      {blogPost.comments && blogPost.comments.length > 0 && (
        <div className="container flex flex-col items-center justify-center py-12 mx-auto">
          <div className="flex flex-col items-center justify-center w-full max-w-2xl">
            <h2 className="mb-4 text-3xl font-bold tracking-tight text-center text-zinc-50 md:text-4xl">
              Comments ({blogPost.comments.length})
            </h2>
            <p className="mb-8 text-lg text-center text-zinc-200">
              {blogPost.comments.map(({ node: comment }) => (
                <div>{comment.author.name}</div>
              ))}
            </p>
          </div>
        </div>
      )}
    </div>
  );
}
