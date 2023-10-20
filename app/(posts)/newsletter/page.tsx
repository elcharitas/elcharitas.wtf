import { executeQuery } from "@/graphql/utils";
import type { SubscribeToNewsletterMutation } from "@/graphql/graphql";
import SubscribeToNewsletter from "@/graphql/mutations/SubscribeToNewsletter.graphql";
import { SubscribeForm } from "./form";

async function subscribe(formData: FormData) {
  "use server";
  const email = formData.get("email")?.valueOf();

  await executeQuery<SubscribeToNewsletterMutation>(
    { SubscribeToNewsletter },
    { input: { email, publicationId: "6231526bc4a093f00c8acd3b" } }
  );
}

export default function Newsletter() {
  return (
    <div className="h-[65vh] flex flex-col justify-center">
      <div className="w-full max-w-md mx-auto space-y-4">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold text-center text-white">
            Subscribe to my newsletter 🔥
          </h1>
          <p className="text-zinc-500 dark:text-zinc-400 text-center">
            Get the latest updates and news delivered to your inbox.
          </p>
        </div>
        <form action={subscribe}>
          <SubscribeForm />
        </form>
      </div>
    </div>
  );
}
