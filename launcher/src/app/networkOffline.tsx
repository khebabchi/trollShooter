"use client";
import { WifiOffIcon } from "lucide-react";
import Image from "next/image";

export default function NetworkOffline() {
  return (
    <>
      <WifiOffIcon
        size={50}
        className="selectDisable text-zinc-400  absolute top-[43%] left-[57%] rotate-[15deg]"
        data-tauri-drag-region
      />
      <div
        data-tauri-drag-region
        className="flex items-center justify-center flex-col gap-3 "
      >
        <Image
          data-tauri-drag-region
          src="/img_app/offline.png"
          alt=""
          width={270}
          height={100}
          className="ml-[-60px] selectDisable"
        />
        <span
          data-tauri-drag-region
          className=" selectDisable text-white flex text-2xl"
        >
          No internet hehe ...
        </span>
      </div>
    </>
  );
}
