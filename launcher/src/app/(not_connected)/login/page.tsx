"use client";
import "../style.css";
import { invoke } from "@tauri-apps/api/tauri";
import Link from "next/link";
import { useRouter } from "next/navigation";
import Logo from "../../logo";
import { useContext } from "react";
import { Button } from "@/components/ui/button";
import Image from "next/image";
import { AppContext } from "@/app/_context/appContext";
export default function Login() {
  if (typeof window !== "undefined") {
    invoke("login");
  }
  const router = useRouter();
   const context= useContext( AppContext);
   const appStarted=context?.appState.appStarted;
  return (
    <div className="signin" data-tauri-drag-region>
      <div className="content" data-tauri-drag-region>
        <Logo appStarted={appStarted} />
        <h2 className="selectDisable" data-tauri-drag-region>
          LogIn
        </h2>

        <div className="form" data-tauri-drag-region>
          <div className="inputBox">
            <input type="text" required /> <i>Username</i>
          </div>

          <div className="inputBox">
            <input type="password" required /> <i>Password</i>
          </div>

          <div className="links">
            <Link draggable="false" className="a" href="#">
              Forgot Password
            </Link>
            <Link draggable="false" className="a" href="/signin">
              Signup
            </Link>
          </div>

          <div className="inputBox flex gap-2 flex-col">
            <Button
              className="input-button input hover:bg-zinc-800"
              color="white"
              onClick={() => {
                router.push("/home");
              }}
            >
              Login
            </Button>
            <Button
              className="w-full text-[1em] hover:bg-zinc-900 hover:border-zinc-500 bg-black border-zinc-600 border-[2px] h-auto py-[7px]"
              onClick={() => {
                router.push("/home");
              }}
            >
              <Image
                src="/img_app/google.png"
                width={17}
                height={17}
                alt=""
                className="mr-4"
              />{" "}
              Connect with google
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
