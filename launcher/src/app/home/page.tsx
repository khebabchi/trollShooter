"use client";
import { redirect } from "next/navigation";
import { User } from "../_context/appContext";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
export default function Home() {
  const [user,setUser]=useState<User>({username:"",email:"",topScore:0,})
  useEffect(()=>{},[setUser])
  invoke("get_user").then((user) =>setUser(user as User))
  return (
    <div className="flex flex-col gap-4 pt-5">
      <div className="flex gap-2">
        <span className="font-bold">Username :</span>
        <span>{user.username}</span>
      </div>
      <div className="flex gap-2">
        <span className="font-bold">Email :</span>
        <span>{user.email}</span>
      </div>
      <div className="flex gap-2">
        <span className="font-bold">Best score :</span>
        <span>{user.topScore} pt</span>
      </div>
    </div>
  );
}
