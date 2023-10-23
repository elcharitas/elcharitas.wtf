import { NextResponse, userAgent } from "next/server";
import type { NextRequest } from "next/server";
import { SinglePostByPublicationQuery } from "./graphql/graphql";
import SinglePostByPublication from "./graphql/queries/SinglePostByPublication.graphql";
import { executeQuery } from "./graphql/utils";

const _sendViewsToHashnodeInternalAnalytics = async (
  publication: SinglePostByPublicationQuery["publication"],
  request: NextRequest,
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
      referrer: request.referrer,
    },
  };

  let deviceId = response.cookies.get("__amplitudeDeviceID")?.value;
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
  request: NextRequest
) => {
  const { post } = publication ?? {};
  if (publication == null || post == null) {
    return;
  }

  const ua = userAgent(request);
  const data = {
    publicationId: publication.id,
    postId: post.id,
    timestamp: Date.now(),
    url: post.url,
    referrer: request.referrer,
    title: post.title,
    charset: request.headers.get("accept-charset") ?? "UTF-8",
    lang: request.headers.get("accept-language") ?? "en-US",
    userAgent: `${ua.browser.name}/${ua.browser.version} (${ua.os.name} ${ua.os.version}; ${ua.device.type})`,
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

  // If the slug is not present, we don't need to do anything
  if (!slug) {
    return response;
  }

  const { data } = await executeQuery<SinglePostByPublicationQuery>(
    { SinglePostByPublication },
    {
      host: "elcharitas.wtf/blog",
      slug,
    }
  );

  // fire and forget the analytics to reduce latency
  _sendViewsToHashnodeInternalAnalytics(
    data.publication,
    request,
    response
  ).catch((error) => {
    console.error("Error sending to Hashnode Internal Analytics:", error);
  });

  // fire and forget the analytics to reduce latency
  _sendViewsToHashnodeAnalyticsDashboard(data.publication, request).catch(
    (error) => {
      console.error("Error sending to Hashnode Analytics Dashboard:", error);
    }
  );

  return response;
}

export const config = {
  matcher: ["/blog/:slug*"],
};
