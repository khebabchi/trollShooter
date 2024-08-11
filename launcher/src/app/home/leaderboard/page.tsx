"use client";
import { fetch } from "@tauri-apps/api/http";
import { useEffect, useState } from "react";
type FetchType={
      username: string;
      topScore: number;
    }[];
export default function Leaderboard() {
  const [fetched, setFetched] = useState<
    FetchType
  >([]);

  async function fetchUsers() {
    try {
      const response = await fetch(
        `https://trollshooterbackend-production.up.railway.app/users`
      );
      if (response.ok) {
        
        setFetched(response.data as FetchType);
      }else{
        console.error(response)
      }
    } catch (error: any) {
      console.log("--------- Getting users : ---------");
      console.error(error);
      console.log("----------------------------------");
    }
  }
  console.log(fetched);
  useEffect(()=>{fetchUsers()},[setFetched])
  const profiles = fetched.map((userInfo, ind) => (
    <Profile
    key={userInfo.username}
      data={{
        classement: ind+1,
        ... userInfo,
      }}
    />
  ));
  return (
    <div className="flex flex-col gap-3 mt-5">
      <div className="flex justify-between px-12 items-center w-[442px] mb-2 text-zinc-300">
        <span className="selectDisable">Username</span>
        <span className="selectDisable">Top score</span>
      </div>
      {profiles}
    </div>
  );
}

function Profile({
  data,
}: {
  data: {
    classement: number;
    username: string;
    topScore: number;
  };
}) {
  const [hovered, setHovered] = useState(false);
  return (
    <div
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      className={`selectDisable border-2 rounded-full w-[435px] h-[45px] flex cursor-pointer p-5 justify-between px-12 bg-zinc-950 text-xs  ${
        hovered ? "text-zinc-200" : "text-zinc-300"
      } flex items-center ${
        data.classement == 1
          ? " gradient-dashed-border"
          : data.classement == 2 || data.classement == 3
          ? "border-[#953ac3a1] hover:border-[#9840c4d0]"
          : "border-zinc-600 hover:border-zinc-500"
      }`}
    >
      <div className="flex gap-2 items-center">
        <span className=" font-bold text-sm">{data.classement}#</span>
        <span>{data.username}</span>
      </div>
      <span
        className={`flex justify-center pr-1 ${
          data.classement == 1
            ? "text-[#cfb35d]"
            : data.classement == 2 || data.classement == 3
            ? "text-[#a95ece]"
            : null
        }`}
      >
        {data.topScore} pt
      </span>
    </div>
  );
}
