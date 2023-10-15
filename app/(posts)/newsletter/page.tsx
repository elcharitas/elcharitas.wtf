"use client";
import Link from "next/link";
import { toast } from "react-hot-toast";

export default function Newsletter() {
  return (
    <div className="h-[50vh] flex flex-col justify-center">
      <div className="w-full max-w-md mx-auto space-y-4">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold text-center text-white">
            Subscribe to my newsletter 🔥
          </h1>
          <p className="text-zinc-500 dark:text-zinc-400 text-center">
            Get the latest updates and news delivered to your inbox.
          </p>
        </div>
        <form className="flex space-x-2">
          <label
            className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            htmlFor="email"
          >
            Email
          </label>
          <input
            className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            id="email"
            placeholder="mail@example.com"
            required
            type="email"
          />
          <button
            type="submit"
            className="flex items-center justify-center h-10 px-4 text-sm font-medium rounded-md border border-transparent bg-zinc-700 hover:bg-zinc-600 text-white disabled:cursor-not-allowed disabled:opacity-50"
            onClick={() => toast.success("Thanks for subscribing! 🎉")}
          >
            Subscribe
          </button>
        </form>
        <p className="text-xs text-zinc-500 dark:text-zinc-400 text-center">
          By subscribing, you agree to our{" "}
          <Link className="underline underline-offset-2" href="#">
            Privacy Policy
          </Link>
        </p>
      </div>
    </div>
  );
}
