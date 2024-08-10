"use client";
import { Inter } from "next/font/google";
import "./globals.css";
const inter = Inter({ subsets: ["latin"] });
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