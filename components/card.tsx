import { PropsWithChildren } from "react";

export const Card: React.FC<PropsWithChildren> = ({ children }) => {
  return (
    <div className="overflow-hidden relative duration-300 ease-out transform rounded-lg hover:bg-zinc-400/10 group md:gap-8 shadow-lg hover:shadow-2xl">
      <div className="pointer-events-none">
        <div className="absolute inset-0 z-0 transition duration-1000 [mask-image:linear-gradient(black,transparent)]" />
        <div className="absolute inset-0 z-10 opacity-100 bg-zinc-400/10 transition duration-1000 group-hover:opacity-50" />
        <div className="absolute inset-0 z-10 opacity-0 mix-blend-overlay transition duration-1000 group-hover:opacity-100" />
      </div>
      {children}
    </div>
  );
};
