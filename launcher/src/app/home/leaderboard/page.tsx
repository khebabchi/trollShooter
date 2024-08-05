"use client";

import { useState } from "react";

export default function Leaderboard() {
  const fetched: {
    name: string;
    achievements: number;
    score: number;
  }[] = [
    {
      name: "hassam_amar",
      achievements: 10,
      score: 2815,
    },
    {
      name: "hassam_amar",
      achievements: 10,
      score: 2815,
    },
    {
      name: "hassam_amar",
      achievements: 10,
      score: 2815,
    },
    {
      name: "hassam_amar",
      achievements: 10,
      score: 2815,
    },
    {
      name: "hassam_amar",
      achievements: 10,
      score: 2815,
    },
    {
      name: "hassam_amar",
      achievements: 10,
      score: 2815,
    },
  ];
  return (
    <div className="flex flex-col gap-3 mt-5">
      <div className="flex justify-around items-center w-[450px] mb-2 text-zinc-300">
        <span className="ml-2 selectDisable">Username</span>
        <span className="ml-3 selectDisable">Top score</span>
        <span className="flex items-center flex-col selectDisable">
          <span>Achievements %</span>
        </span>
      </div>
      <Profile
        data={{
          classement: 1,
          ...fetched[0],
        }}
      />
      <Profile
        data={{
          classement: 2,
          ...fetched[1],
        }}
      />
      <Profile
        data={{
          classement: 3,
          ...fetched[2],
        }}
      />
      <Profile
        data={{
          classement: 4,
          ...fetched[3],
        }}
      />{" "}
      <Profile
        data={{
          classement: 5,
          ...fetched[4],
        }}
      />{" "}
      <Profile
        data={{
          classement: 6,
          ...fetched[5],
        }}
      />
    </div>
  );
}

function Profile({
  data,
}: {
  data: {
    classement: number;
    name: string;
    achievements: number;
    score: number;
  };
}) {
  const [hovered, setHovered] = useState(false);
  return (
    <div
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      className={`selectDisable border-2 rounded-full w-[440px] h-[45px] flex cursor-pointer p-5 bg-zinc-950 text-xs  ${
        hovered ? "text-zinc-200" : "text-zinc-300"
      } flex items-center ${
        data.classement == 1
          ? " gradient-dashed-border"
          : data.classement == 2 || data.classement == 3
          ? "border-[#953ac3a1] hover:border-[#9840c4d0]"
          : "border-zinc-600 hover:border-zinc-500"
      }`}
    >
      <div className="flex gap-2 items-center w-[140px] ">
        <span className=" font-bold text-sm">{data.classement}#</span>
        <span>{data.name}</span>
      </div>
      <span
        className={`w-[100px] mr-12 flex justify-center ${
          data.classement == 1
            ? "text-[#cfb35d]"
            : data.classement == 2 || data.classement == 3
            ? "text-[#a95ece]"
            : null
        }`}
      >
        {data.score} pt
      </span>
      <span className="w-[90px] flex justify-center text-zinc-400">{data.achievements}%</span>
    </div>
  );
}
