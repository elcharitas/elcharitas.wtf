import { PostsPage } from "../post-page";

const featured = {
  slug: "rustle-rs",
  title: "Rustle.rs",
  description: "A rust framework for building web applications",
  date: "2021-08-01",
};

export default async function Page() {
  return <PostsPage title="Talks" featured={featured} />;
}
