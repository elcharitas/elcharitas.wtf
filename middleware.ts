import { NextResponse } from "next/server";
import { kv } from "@vercel/kv";
import type { NextRequest } from "next/server";

export async function middleware(request: NextRequest) {
  const { nextUrl } = request;
  const slug = nextUrl.pathname.split("/").pop();
  const response = NextResponse.next();

  // ensure we track one view per request
  if (!response.cookies.get("blog-" + slug + "-viewed")?.value) {
    const currentViews = (await kv.get<number>(`blog-${slug}-views`)) || 0;
    await kv.set(`blog-${slug}-views`, currentViews + 1);
    response.cookies.set("blog-" + slug + "-viewed", "1", {
      maxAge: 60 * 60 * 24 * 365, // 1 year
      path: "/",
    });
  }

  return response;
}

export const config = {
  matcher: ["/blog/:slug*"],
};
