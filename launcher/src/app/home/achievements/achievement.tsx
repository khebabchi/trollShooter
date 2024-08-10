import Image from "next/image";
import { LockIcon } from "lucide-react";
import { useState } from "react";
export interface achievement {
  img: string;
  title: string;
  description: string;
  rarity: number;
  done: boolean;
}
export default function Achievement({ data }: { data: achievement }) {
  const [hovered, setHovered] = useState(false);
  return (
    <div>
      {!data.done && (
        <LockIcon
          size={18}
          className={`selectDisable text-black ${
            hovered ? "bg-zinc-50" : "bg-zinc-500"
          } rounded p-[2px] relative top-[11px] left-3 z-20 border-2 border-black`}
        />
      )}
      <div
        onMouseEnter={() => setHovered(true)}
        onMouseLeave={() => setHovered(false)}
        className={`selectDisable border-2 rounded-xl w-[480px] h-[72px] cursor-pointer p-2 bg-zinc-950 text-xs ${
          !data.done && "mt-[-18px]"
        } ${
          hovered ? "text-zinc-200" : "text-zinc-300"
        } flex justify-between items-center ${
          !data.done ? (hovered ? "opacity-65" : "opacity-45") : ""
        } ${
          data.rarity == 3
            ? " gradient-dashed-border"
            : data.rarity == 2
            ? "border-[#953ac3a1] hover:border-[#9840c4d0]"
            : "border-zinc-600 hover:border-zinc-500"
        }`}
      >
        <div className="selectDisable  flex items-center gap-2">
          <Image
            src={`/achievements/${data.img}`}
            alt="alt"
            width={30}
            height={30}
            className=" selectDisable w-14 h-14 flex it justify-center items-center border-2 border-zinc-800 rounded-md"
          />
          <div className="selectDisable flex flex-col text-sm gap-[2px]">
            <span>{data.title}</span>
            <span className=" opacity-50">{data.description}</span>
          </div>
        </div>
        <span
          className={`mr-4 w-[70px] flex justify-center gap-1 font-bold items-center ${
            data.rarity == 3
              ? "text-[#cfb35d]"
              : data.rarity == 2
              ? "text-[#9840c4d0]"
              : null
          }`}
        >
          {data.rarity == 3
            ? "Legandary"
            : data.rarity == 2
            ? "Rare"
            : "Commun"}
        </span>
      </div>
    </div>
  );
}
