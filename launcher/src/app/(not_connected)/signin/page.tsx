"use client";
import "../style.css";
import logo from "@/app/img/icon.png";
import { invoke } from "@tauri-apps/api/tauri";
import Image from "next/image";
import Link from "next/link";
export default function Home() {
  if (typeof window !== "undefined") {
    invoke("signin");
  }

  return (
    <div className="signin" data-tauri-drag-region>
      <div className="content" data-tauri-drag-region>
        <Image src={logo} alt="" />
        <h2>Sign In</h2>

        <div className="form" data-tauri-drag-region>
          <div className="inputBox">
            <input type="text" required /> <i>Email</i>
          </div>

          <div className="inputBox">
            <input type="password" required /> <i>Username</i>
          </div>
          <div className="inputBox">
            <input type="password" required /> <i>Password</i>
          </div>
          <div className="inputBox">
            <input type="password" required /> <i>Confirm password</i>
          </div>

          <div className="links">
            <div className=" flex ">
              <span className="a hover:bg-black opacity-60 w-full h-auto mr-2">
                Already have an account ?
              </span>
              <Link className="a" href="/login">
                Login
              </Link>
            </div>
          </div>

          <div className="inputBox">
            <input type="submit" value="Signin" />
          </div>
        </div>
      </div>
    </div>
  );
}
