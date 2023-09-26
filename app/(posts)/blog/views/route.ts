import { NextResponse } from "next/server";
import { kv } from "@vercel/kv";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const slug = searchParams.get("slug");

  return NextResponse.json({
    views: await kv.get<number>(`${slug}-views`),
  });
}
