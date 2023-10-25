import { PostsPage } from "@/components/post-page";
import { getAllProjects } from "./util";

export const revalidate = 60 * 60 * 24 * 7;

export default async function Page() {
  const [featured, top1, top2, ...projects] = await getAllProjects();
  return (
    <PostsPage
      title="Projects ⚡️"
      count={projects.length + 3}
      description="These are some of the amazing projects I've worked or collaborated on. They're all on my github profile but require a stargazer or more to show up here 😅."
      featured={[featured, top1, top2]}
      sorted={projects}
    />
  );
}
