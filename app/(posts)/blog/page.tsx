import Blogs from "./blogs";
import { getAllBlogs } from "./utils";

export const revalidate = 60;

export default async function Page() {
  const blogPosts = await getAllBlogs();
  return <Blogs initialPosts={blogPosts} />;
}
