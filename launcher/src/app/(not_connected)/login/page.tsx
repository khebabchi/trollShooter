"use client";
import "../style.css";
import logo from "@/app/img/icon.png";
import { invoke } from "@tauri-apps/api/tauri";
import Image from "next/image";
import Link from "next/link";
export default function Home() {
  if (typeof window !== "undefined") {
    invoke("login");
  }

  return (
    <div className="signin" data-tauri-drag-region>
      <div className="content" data-tauri-drag-region>
        <Image
          className="selectDisable"
          data-tauri-drag-region
          draggable="false"
          src={logo}
          alt=""
        />
        <h2 className="selectDisable" data-tauri-drag-region>
          Sign In
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
            </Link>{" "}
            <Link draggable="false" className="a" href="/signin">
              Signup
            </Link>
          </div>

          <div className="inputBox">
            <input type="submit" value="Login" />
          </div>
        </div>
      </div>
    </div>
  );
}
