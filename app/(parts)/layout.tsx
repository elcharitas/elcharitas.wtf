import { Navigation } from "@/components/nav";
import { PropsWithChildren } from "react";

export default function ProjectsLayout({ children }: PropsWithChildren) {
  return (
    <div className="relative min-h-screen bg-gradient-to-tl from-black via-zinc-600/20 to-black ">
      <div className="relative pb-16">
        <Navigation />
        <div className="px-6 pt-12 md:mx-auto space-y-8 max-w-7xl lg:px-8 md:space-y-16 md:pt-24 lg:pt-32">
          {children}
        </div>
      </div>
      <div className="flex justify-center text-sm text-zinc-50 py-8 border-t border-zinc-800">
        Designed and developed by &nbsp;
        <a
          href="https://elcharitas.wtf"
          className="font-medium text-zinc-500 hover:text-zinc-300"
        >
          Jonathan Irhodia
        </a>
        &nbsp; © {new Date().getFullYear()}
      </div>
    </div>
  );
}
