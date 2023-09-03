import { Card } from "@/app/components/card";
import adventures from "./adventures.json";

const adventuresByYear = Object.entries(adventures)
  .map(([year, yearData]) => ({
    year,
    yearData: Object.entries(yearData).map(([month, info]) => ({
      month,
      info,
    })),
  }))
  .reverse();

export default async function Page() {
  return (
    <>
      <div className="max-w-2xl mx-auto lg:mx-0">
        <h2 className="text-3xl font-bold tracking-tight text-zinc-100 sm:text-4xl">
          Adventures
        </h2>
        <p className="mt-4 text-zinc-400">
          Here&apos;s some of the more exciting things I&apos;ve done or
          experienced
        </p>
      </div>
      <div className="w-full h-px bg-zinc-800" />
      <div className="max-w-full w-full mx-auto lg:mx-0">
        <ul className="mt-4 text-zinc-400">
          {adventuresByYear.map(({ year, yearData }) => (
            <li key={year} className="mb-2 flex flex-col items-center">
              <div className="flex flex-col md:w-3/4">
                <span className="text-zinc-500 font-display text-4xl text-center my-8 py-8 border-zinc-800 border rounded-xl">
                  {year}
                </span>
                {yearData.map(({ month, info }, index) => (
                  <div
                    className={`mb-4 md:ml-4 md:mt-8 md:flex flex-col ${
                      index % 2 ? "items-start" : "items-end"
                    }`}
                  >
                    <Card>
                      <div className="m-4 min-w-[300px]">
                        <h3 className="text-lg font-display uppercase">
                          {month}
                        </h3>
                        {info.map((data) => (
                          <p key={data} className="text-zinc-400">
                            &#8226; &nbsp; {data}
                          </p>
                        ))}
                      </div>
                    </Card>
                  </div>
                ))}
              </div>
            </li>
          ))}
        </ul>
      </div>
      <div className="w-full h-px bg-zinc-800" />
    </>
  );
}
