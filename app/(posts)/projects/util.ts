export async function getAllProjects(page = 1): Promise<Post[]> {
  const response = await fetch(
    `https://api.github.com/user/repos?sort=updated&visibility=public&affiliation=owner,collaborator&per_page=100&direction=desc&page=${page}`,
    {
      headers: {
        Authorization: `token ${process.env.GITHUB_TOKEN}`,
      },
    }
  );
  const data = await response.json();

  const filteredData: Repo[] =
    data.filter?.(
      (repo: Repo) => !repo.fork && repo.description && repo.stargazers_count
    ) ?? [];

  return Promise.all(
    filteredData
      .sort((a, b) => b.stargazers_count - a.stargazers_count)
      .map(async (repo) => {
        const coverImageUrl = `https://raw.githubusercontent.com/${repo.owner.login}/${repo.name}/${repo.default_branch}/static/logo-light.svg`;
        const isCoverImage =
          (await fetch(coverImageUrl, { method: "HEAD" })).status === 200;
        return {
          slug: repo.name,
          title: repo.name.replace(/[-_]/g, " "),
          brief: repo.description,
          date: repo.pushed_at,
          url: repo.homepage ?? "",
          content: "",
          coverImage: isCoverImage ? coverImageUrl : undefined,
          type: "projects",
          views: repo.stargazers_count,
          owner: repo.owner.login,
        };
      })
  );
}
