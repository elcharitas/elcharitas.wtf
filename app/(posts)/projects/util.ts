export async function getAllProjects(): Promise<Post[]> {
  const response = await fetch(
    "https://api.github.com/user/repos?sort=stars&affiliation=owner,collaborator&per_page=350&direction=desc",
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
          title: `${repo.owner.login}/${repo.name}`,
          brief: repo.description,
          date: repo.pushed_at,
          url: repo.html_url,
          content: "",
          coverImage: isCoverImage ? coverImageUrl : null,
          type: "projects",
          views: repo.stargazers_count,
          owner: repo.owner.login,
        };
      })
  );
}
