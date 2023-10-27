import { FC } from "react";
import { Card } from "@/components/card";
import { Content } from "@/components/content";

interface AdventureProps {
  year: string;
  yearData: {
    month: string;
    info: string[];
  }[];
}
const Adventure: FC<AdventureProps> = ({ year, yearData }) => {
  return (
    <li className="mb-2 flex flex-col items-center">
      <div className="flex flex-col md:w-3/4">
        <div
          tabIndex={0}
          role="button"
          className="text-zinc-500 font-display animate-title text-center text-4xl my-8 py-4 duration-700 hover:text-yellow-500/80 hover:scale-110"
        >
          {year}
        </div>
        {yearData.map(({ month, info }, pos) => (
          <div
            key={month}
            className={`mb-4 md:ml-4 md:mt-8 animate-fade-in md:flex flex-col ${
              pos % 2 ? "items-start" : "items-end"
            }`}
          >
            <Card>
              <div className="p-4 max-w-[500px] min-w-[300px] md:w-[500px]">
                <div className="px-4 pb-4 w-full">
                  <h3 className="text-lg font-display uppercase my-2 text-yellow-500/80">
                    {month}
                  </h3>
                  {info.map((data) => (
                    <p key={data} className="text-zinc-400">
                      <span className="text-yellow-500">&#8226;</span> &nbsp;{" "}
                      <Content text={data} />
                    </p>
                  ))}
                </div>
              </div>
            </Card>
          </div>
        ))}
      </div>
    </li>
  );
};

export { Adventure };
