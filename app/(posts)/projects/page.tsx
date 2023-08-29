import { PostsPage } from "../post-page";

export default async function Page() {
  const response = await fetch(
    "https://api.github.com/user/repos?sort=stars&affiliation=owner,collaborator&per_page=150&direction=desc",
    {
      headers: {
        Authorization: `token ${process.env.GITHUB_TOKEN}`,
      },
    }
  );
  const data = await response.json();

  const filteredData: Repo[] =
    data.filter?.((repo: Repo) => !repo.fork && repo.stargazers_count) ?? [];

  const [featured, top1, top2, ...projects]: Post[] = filteredData
    .sort((a, b) => b.stargazers_count - a.stargazers_count)
    .map((repo) => ({
      slug: repo.name,
      title: `${repo.owner.login}/${repo.name}`,
      brief: repo.description,
      date: repo.pushed_at,
      url: repo.html_url,
      content: "",
      coverImage: repo.owner.avatar_url,
      type: "projects",
      views: repo.stargazers_count,
    }));

  return (
    <PostsPage
      title="Projects"
      description="As a software engineer, I have worked on some amazing projects. This is a list of my favorite ones."
      featured={[featured, top1, top2]}
      sorted={projects}
    />
  );
}
