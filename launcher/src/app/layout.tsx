"use client";
import { Inter } from "next/font/google";
import "./globals.css";
const inter = Inter({ subsets: ["latin"] });
import { Minus, X } from "lucide-react";
import { createContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import useNetworkStatus from "./useNetworkStatus";
import NetworkOffline from "./networkOffline";
import { usePathname } from "next/navigation";
import { AppProvider } from "./_context/appContext";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className + " bg-black"} data-tauri-drag-region>
        <AppProvider>{children}</AppProvider>
      </body>
    </html>
  );
}
