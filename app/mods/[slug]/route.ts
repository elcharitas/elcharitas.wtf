import { get } from "@vercel/edge-config";
import { notFound } from "next/navigation";

export async function GET(request: Request) {
  const { pathname } = new URL(request.url);
  const modId = pathname.split("/")[2];
  const modUrl = await get(modId);

  if (modUrl) {
    try {
      return Response.redirect(String(modUrl), 302);
    } catch (_) {
      return notFound();
    }
  }

  return notFound();
}
