"use client";
import { Inter } from "next/font/google";
import "./globals.css";
const inter = Inter({ subsets: ["latin"] });
import { Minus, X } from "lucide-react";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  useEffect(() => {
    document.body.style.overflow = "hidden";
    invoke("init");
  });
  return (
    <html lang="en">
      <body className={inter.className}>
        {typeof window !== "undefined" && (
          <main
            className="flex flex-col items-center bg-black h-screen justify-center"
            data-tauri-drag-region="true"
          >
            <span
              data-tauri-drag-region
              className=" text-white bg-black opacity-90 absolute top-3 left-4 z-10  w-fit selectDisable"
            >
              Survive the troll
            </span>
            <div className="flex items-center justify-between gap-1 absolute top-3 right-3">
              <Minus
                className=" text-white hover:opacity-70 hover:cursor-pointer z-10"
                onClick={() => {
                  invoke("minimize_window");
                }}
              />
              <X
                className=" text-white hover:opacity-70 hover:cursor-pointer z-10"
                onClick={() => {
                  invoke("close_window");
                }}
              />
            </div>
            {children}
          </main>
        )}
      </body>
    </html>
  );
}
