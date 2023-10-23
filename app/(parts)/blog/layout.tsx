import Link from "next/link";
import { MailPlus } from "lucide-react";
import { PropsWithChildren } from "react";

export default function BlogLayout({ children }: PropsWithChildren) {
  return (
    <>
      {children}
      <Link
        href="/newsletter"
        className="text-sm duration-500 text-zinc-50 hover:text-zinc-100 flex items-center justify-center gap-2 bg-zinc-700 hover:bg-zinc-600 rounded-md px-4 py-2 fixed bottom-8 right-8 hover:scale-110"
      >
        <MailPlus /> Subscribe
      </Link>
    </>
  );
}
