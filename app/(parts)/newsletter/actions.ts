"use server";
import { executeQuery } from "@/graphql/utils";
import type { SubscribeToNewsletterMutation } from "@/graphql/graphql";
import SubscribeToNewsletter from "@/graphql/mutations/SubscribeToNewsletter.graphql";
import { publicationId } from "@/graphql/config";

export async function subscribe(formData: FormData) {
  const email = formData.get("email")?.valueOf();

  await executeQuery<SubscribeToNewsletterMutation>(
    { SubscribeToNewsletter },
    { input: { email, publicationId } }
  );
}
