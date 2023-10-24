import Blogs from "./blogs";
import { getAllBlogs } from "./utils";

export const revalidate = 60 * 60 * 24; // 1 day;

export default async function Page() {
  const [blogPosts, initialCursor] = await getAllBlogs();
  return <Blogs initialPosts={blogPosts} initialCursor={initialCursor} />;
}
