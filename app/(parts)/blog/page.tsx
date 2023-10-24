import Blogs from "./blogs";
import { getAllBlogs } from "./utils";

export const revalidate = 60 * 60 * 24 * 7; // 1 week;

export default async function Page() {
  const [blogPosts, initialCursor] = await getAllBlogs();
  return <Blogs initialPosts={blogPosts} initialCursor={initialCursor} />;
}
