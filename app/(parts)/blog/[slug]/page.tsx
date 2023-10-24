import { notFound } from "next/navigation";
import { Mdx } from "@/components/mdx";
import { Header } from "@/components/header";
import { getBlogPost } from "../utils";

export const revalidate = 60 * 30; // 30 min

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
    </div>
  );
}
