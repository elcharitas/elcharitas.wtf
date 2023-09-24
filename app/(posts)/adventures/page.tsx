import { Card } from "@/app/components/card";
import adventures from "./adventures.json";
import { Adventure } from "./adventure";

const adventuresByYear = Object.entries(adventures)
  .map(([year, yearData]) => ({
    year,
    yearData: Object.entries(yearData)
      .map(([month, info]) => ({
        month,
        info,
      }))
      .reverse(),
  }))
  .reverse();

export default async function Page() {
  return (
    <>
      <div className="max-w-2xl mx-auto lg:mx-0">
        <h2 className="text-3xl font-bold tracking-tight text-zinc-100 sm:text-4xl">
          Adventures 🔥
        </h2>
        <p className="mt-4 text-zinc-400">
          I&apos;ve been fortunate to travel and work on incredible projects
          with amazing people. 🤌
        </p>
      </div>
      <div className="w-full h-px bg-zinc-800" />
      <div className="max-w-full w-full mx-auto lg:mx-0">
        <ul className="mt-4 text-zinc-400">
          {adventuresByYear.map(({ year, yearData }) => (
            <Adventure key={year} year={year} yearData={yearData} />
          ))}
        </ul>
      </div>
      <div className="w-full h-px bg-zinc-800" />
    </>
  );
}
