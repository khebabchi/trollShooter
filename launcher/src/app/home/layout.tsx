"use client";
import Logo from "../logo";
import Link from "next/link";
import { AwardIcon, SettingsIcon, UngroupIcon } from "lucide-react";
import { useContext, useRef } from "react";
import { redirect, usePathname } from "next/navigation";
import { AppContext, User } from "../_context/appContext";
export default function HomeLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const context = useContext(AppContext);
  const appStarted = context?.appState.appStarted == true;
  const invoked = useRef(false);
  if (typeof window !== "undefined" && !invoked.current) {
    window.__TAURI__.tauri.invoke("home");
    invoked.current = true;
  }
  const path = usePathname();
  window.__TAURI__.tauri.invoke("get_user").then((user:User) => {
    if (!user.username) {
      console.info(user);
      redirect("/login");
    }
  });
  const linkClasses = (href: string): string =>
    `flex selectDisable transition-opacity hover:opacity-100 selectDisable ${
      path == href
        ? " opacity-100 underline underline-offset-[10px]"
        : " opacity-50"
    }`;
  return (
    <>
      {path != "/home/goplay" ? (
        <div
          className="flex flex-col h-full w-full pt-10 pb-16 gap-5 items-center"
          data-tauri-drag-region
        >
          <Logo big appStarted={appStarted} />
          <div
            className="flex gap-7 items-center text-white selectDisable"
            data-tauri-drag-region
          >
            <Link
              href="/play/index.html"
              onClick={() => window.__TAURI__.tauri.invoke("play")}
              className="selectDisable py-[6px] bg-white text-black hover:opacity-70 transition-opacity  font-semibold text-lg rounded-xl px-5 mr-5"
            >
              Play
            </Link>
            <Link
              draggable={false}
              className={linkClasses("/home")}
              href="/home"
            >
              <SettingsIcon size={23} />
              <span className="ml-[-22px]">
                &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Profile
              </span>
            </Link>
            <Link
              draggable={false}
              className={linkClasses("/home/achievements")}
              href="/home/achievements"
            >
              <AwardIcon size={23} />
              <span className="ml-[-22px]">
                &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Achievements
              </span>
            </Link>
            <Link
              draggable={false}
              className={linkClasses("/home/leaderboard")}
              href="/home/leaderboard"
            >
              <UngroupIcon size={23} />
              <span className="ml-[-22px]">
                &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Leaderboard
              </span>
            </Link>
          </div>
          <div className="text-white" data-tauri-drag-region>
            {children}
          </div>
        </div>
      ) : (
        children
      )}
    </>
  );
}
