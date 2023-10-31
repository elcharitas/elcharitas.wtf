import { revalidatePath } from "next/cache";
import { NextResponse } from "next/server";

interface WebhookData {
  metadata: {
    uuid: string;
  };
  data: {
    eventType: "post_published" | "post_updated" | "post_deleted";
    publication: {
      id: string;
    };
    post: {
      id: string;
    };
  };
}

export async function POST(request: Request) {
  const body: WebhookData = await request.json();

  if (
    body.data.eventType === "post_published" ||
    body.data.eventType === "post_deleted"
  ) {
    revalidatePath("/blog");
    revalidatePath("/blog/:slug");
  }

  // const post = await getBlogPost(body.data.post.id);
  // revalidatePath(`/blog/${post.slug}`);

  return NextResponse.json({
    message: "revalidated",
  });
}
