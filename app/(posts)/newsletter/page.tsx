import { SubscribeForm } from "./form";
import { subscribe } from "./actions";

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
