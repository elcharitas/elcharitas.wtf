import { getBlogPost } from "@/app/(parts)/blog/utils";
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
  const post = await getBlogPost(body.data.post.id);

  if (
    body.data.eventType === "post_published" ||
    body.data.eventType === "post_deleted"
  ) {
    revalidatePath("/blog");
  }

  revalidatePath(`/blog/${post.slug}`);

  return NextResponse.json({
    message: "revalidated",
  });
}
