"use client";
import { toast } from "react-hot-toast";
import { experimental_useFormStatus as useFormStatus } from "react-dom";
import { useEffect, useMemo, useRef } from "react";
import Link from "next/link";

export const SubscribeForm = () => {
  const { pending } = useFormStatus();
  const pendingRef = useRef(pending);

  const isSubmitted = useMemo(() => {
    return pendingRef.current && !pending;
  }, [pending, pendingRef.current]);

  useEffect(() => {
    pendingRef.current = pending;
    if (isSubmitted) {
      pendingRef.current = false;
      toast.success("Thanks for subscribing!");
    }
  }, [isSubmitted, pending, pendingRef.current]);

  if (isSubmitted) {
    return (
      <p className="text-xs text-zinc-500 dark:text-zinc-400 text-center">
        Thanks for subscribing! A confirmation email has been sent to your
        inbox. If you don&apos;t see it, check your spam folder. 📬
      </p>
    );
  }

  return (
    <>
      <div className="flex space-x-2 mb-4">
        <label className="sr-only" htmlFor="email">
          Email
        </label>
        <input
          className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
          id="email"
          name="email"
          placeholder="mail@example.com"
          required
          type="email"
          data-tour="email-form"
          data-tour-title="Subscribe to my newsletter"
          data-tour-description="We'll send you an email to confirm your subscription."
          data-tour-position="right"
        />
        <button
          type="submit"
          className="flex items-center justify-center h-10 px-4 text-sm font-medium rounded-md border border-transparent bg-zinc-700 hover:bg-zinc-600 text-white disabled:cursor-not-allowed disabled:opacity-50"
          disabled={pending}
          data-tour="Subscribe"
          data-tour-description="Click here to subscribe to my newsletter."
          data-tour-position="right"
        >
          Subscribe
        </button>
      </div>
      <p className="text-xs text-zinc-500 dark:text-zinc-400 text-center">
        By subscribing, you agree to our{" "}
        <Link className="underline underline-offset-2" href="#">
          Privacy Policy
        </Link>
      </p>
    </>
  );
};
