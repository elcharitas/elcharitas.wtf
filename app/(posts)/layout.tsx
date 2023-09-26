import { Navigation } from "../components/nav";

export default function ProjectsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="relative min-h-screen bg-gradient-to-tl from-black via-zinc-600/20 to-black ">
      <div className="relative pb-16">
        <Navigation />
        <div className="px-6 pt-12 md:mx-auto space-y-8 max-w-7xl lg:px-8 md:space-y-16 md:pt-24 lg:pt-32">
          {children}
        </div>
      </div>
    </div>
  );
}
