import { notFound } from "next/navigation";
import { Mdx } from "@/app/components/mdx";
import { Header } from "../../header";
import { getAllProjects } from "../util";
import "../../mdx.css";

export const revalidate = 60;

type Props = {
  params: {
    slug: string;
  };
};

export async function generateStaticParams(): Promise<Props["params"][]> {
  const allProjects = await getAllProjects();
  return allProjects.map((p) => ({
    slug: p.slug,
  }));
}

export default async function PostPage({ params }: Props) {
  const allProjects = await getAllProjects();
  const project = allProjects.find((project) => project.slug === params.slug);

  if (!project) {
    notFound();
  }

  const response = await fetch(
    `https://api.github.com/repos/${project.owner}/${project.slug}/readme`,
    {
      headers: {
        Authorization: `token ${process.env.GITHUB_TOKEN}`,
      },
    }
  );
  const data = await response.json();

  return (
    <div className="bg-zinc-900 min-h-screen">
      <Header post={project} />

      {data.content && (
        <article className="px-4 md:px-0 py-12 md:mx-auto prose prose-zinc prose-quoteless">
          <Mdx
            baseUri={`https://raw.githubusercontent.com/${project.owner}/${project.slug}/${project.branch}/`}
            code={Buffer.from(data.content, data.encoding).toString()}
          />
        </article>
      )}
    </div>
  );
}
