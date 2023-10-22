import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { SinglePostByPublicationQuery } from "./graphql/graphql";
import SinglePostByPublication from "./graphql/queries/SinglePostByPublication.graphql";
import { executeQuery } from "./graphql/utils";

const _sendViewsToHashnodeInternalAnalytics = async (
  publication: SinglePostByPublicationQuery["publication"],
  response: NextResponse
) => {
  if (publication == null || publication.post == null) {
    return;
  }
  // Send to Hashnode's own internal analytics
  const event: Record<string, string | number | object> = {
    event_type: "pageview",
    time: new Date().getTime(),
    event_properties: {
      hostname: "elcharitas.wtf",
      url: publication.post.url,
      eventType: "pageview",
      publicationId: publication.id,
      dateAdded: new Date().getTime(),
      referrer: "",
    },
  };

  let deviceId = Math.random().toString(36).substring(2, 15); //response.cookies.get("__amplitudeDeviceID")?.value;
  if (!deviceId) {
    deviceId = Math.random().toString(36).substring(2, 15);
    response.cookies.set("__amplitudeDeviceID", deviceId, {
      expires: 365 * 2,
    }); // expire after two years
  }

  event["device_id"] = deviceId;

  return await fetch("https://hn-ping2.hashnode.com/api/data-event", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ events: [event] }),
  });
};

const _sendViewsToHashnodeAnalyticsDashboard = async (
  publication: SinglePostByPublicationQuery["publication"],
  slug?: string
) => {
  const { post } = publication ?? {};
  if (publication == null || post == null) {
    return;
  }

  const data = {
    publicationId: publication.id,
    postId: post.id,
    timestamp: Date.now(),
    url: post.url,
    referrer: "",
    title: post.title,
    charset: "UTF-8",
    lang: "en-US",
    userAgent:
      "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    historyLength: 1,
    timezoneOffset: new Date().getTimezoneOffset(),
  };

  // For Hashnode Blog Dashboard Analytics
  return await fetch("https://hn-ping2.hashnode.com/api/view", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ data }),
  });
};

export async function middleware(request: NextRequest) {
  const { nextUrl } = request;
  const slug = nextUrl.pathname.split("/").pop();
  const response = NextResponse.next();

  const { data } = await executeQuery<SinglePostByPublicationQuery>(
    { SinglePostByPublication },
    {
      host: "elcharitas.wtf/blog",
      slug,
    }
  );

  await _sendViewsToHashnodeInternalAnalytics(data.publication, response);
  await _sendViewsToHashnodeAnalyticsDashboard(data.publication, slug);

  return response;
}

export const config = {
  matcher: ["/blog/:slug*"],
};
