import { PostsPage } from "../post-page";

export default async function Page() {
  const response = await fetch(
    "https://api.github.com/users/elcharitas/repos?sort=stars"
  );
  const data = await response.json();

  const filteredData = data.filter((repo: Repo) => !repo.fork);

  const [featured, ...projects] = filteredData.map((repo: Repo) => ({
    slug: repo.name,
    title: repo.name,
    brief: repo.description,
    date: repo.created_at,
    url: repo.html_url,
    content: "",
  }));

  return <PostsPage title="Projects" featured={featured} sorted={projects} />;
}
