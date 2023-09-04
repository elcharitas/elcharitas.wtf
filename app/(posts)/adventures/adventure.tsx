"use client";
import { useState, FC } from "react";
import { Card } from "@/app/components/card";

interface AdventureProps {
  year: string;
  yearData: {
    month: string;
    info: string[];
  }[];
}
const Adventure: FC<AdventureProps> = ({ year, yearData }) => {
  const [isExpanded, setIsExpanded] = useState(true);
  return (
    <li className="mb-2 flex flex-col items-center">
      <div className="flex flex-col md:w-3/4">
        <div
          tabIndex={0}
          role="button"
          className="text-zinc-500 font-display animate-title text-4xl text-center my-8 py-8 border-zinc-800 border rounded-xl"
          onClick={() => setIsExpanded((prev) => !prev)}
        >
          {year}
        </div>
        {isExpanded &&
          yearData.map(({ month, info }, index) => (
            <div
              className={`mb-4 md:ml-4 md:mt-8 animate-fade-in md:flex flex-col ${
                index % 2 ? "items-start" : "items-end"
              }`}
            >
              <Card>
                <div className="m-4 min-w-[300px]">
                  <h3 className="text-lg font-display uppercase">{month}</h3>
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
  );
};

export { Adventure };
