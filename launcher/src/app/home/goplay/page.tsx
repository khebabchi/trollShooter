"use client";
import { redirect } from "next/navigation";
import { invoke } from "@tauri-apps/api/tauri";
export default function Home() {
  
  console.log("invoked play");
 

  return <>{invoke("play")}{ redirect("/play/index.html")}</>;
}
