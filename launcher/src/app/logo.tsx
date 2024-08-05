"use client";
import Image from "next/image";
import { useEffect, useState } from "react";

export default function Logo({
  big = false,
  appStarted = true,
}: {
  big?: boolean;
  appStarted: boolean | undefined;
}) {
  const [isHovered, setIsHovered] = useState(!big);
  useEffect(() => {
    appStarted && setIsHovered(false);
  }, [appStarted]);

  return (
    <div className={`flex gap-10 items-center ${big && "py-10"}`}>
      <Image
        onMouseEnter={() => {
          appStarted && setIsHovered(true);
        }}
        onMouseLeave={() => {
          appStarted && setIsHovered(false);
        }}
        className="selectDisable"
        data-tauri-drag-region
        draggable="false"
        height={100}
        width={100}
        src={`/img_app/${
          isHovered || !appStarted
            ? big
              ? "icon_scary_inv"
              : "icon_scary"
            : big
            ? "icon_inv"
            : "icon"
        }.png`}
        alt=""
      />
      {big && (
        <Image
          className="selectDisable object-contain"
          data-tauri-drag-region
          draggable="false"
          height={45}
          width={270}
          src={`/img_app/${
            isHovered || !appStarted ? "title_scary" : "title"
          }.png`}
          alt=""
        />
      )}
    </div>
  );
}
