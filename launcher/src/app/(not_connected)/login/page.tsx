"use client";
import "../style.css";
import Link from "next/link";
import { useRouter } from "next/navigation";
import Logo from "../../logo";
import { useContext, useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { AppContext, User } from "@/app/_context/appContext";
import { invoke } from "@tauri-apps/api/tauri";
import { Body, fetch } from "@tauri-apps/api/http";
//import { Body} from "@tauri-apps/api/http";
export default function Login() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
  if (typeof window !== "undefined") {
    invoke("login");
    console.log("window.__TAURI__.http");
  }

  const router = useRouter();
  const context = useContext(AppContext);
  const appStarted = context?.appState.appStarted;
  async function handleLogin() {
    setErrorMessage("");
    const url = `https://trollshooterbackend-production.up.railway.app/user/login`;
    try {
      const response = await fetch(url, {
        method: "POST",
        headers: {
          "Content-Type": "application/json", // Correctly set the content type
        },
        body: Body.json({
          username,
          password,
        }),
      });
      if (!response.ok) {
        setErrorMessage("Username/password incorrect");
      } else {
        const user: User = response.data as User;
        if (!user) {
          setErrorMessage("Username/password incorrect");
        } else {
          invoke("set_user", { user });

          console.log(user);
          router.push("/home");
        }
      }
    } catch (error: any) {
      console.log("------------------");
      console.error(error);
      setErrorMessage("Username/password incorrect");
      console.log("------------------");
    }
  }
  //------------------------------------

  useEffect(() => {
    function handleKeyPress(event:KeyboardEvent) {
      if (event.key === "Enter") {
        handleLogin();
      }
    }

    window.addEventListener("keydown", handleKeyPress);
    
    return () => {
      window.removeEventListener("keydown", handleKeyPress);
    };
  });

  //------------------------------------
  return (
    <div className="signin" data-tauri-drag-region>
      <div className="content" data-tauri-drag-region>
        <Logo appStarted={appStarted} />
        <h2 className="selectDisable" data-tauri-drag-region>
          LogIn
        </h2>

        <div className="form" data-tauri-drag-region>
          <div className="inputBox">
            <input
              type="text"
              required
              value={username}
              onChange={(e) => setUsername(e.target.value)}
            />{" "}
            <i>Username</i>
          </div>

          <div className="inputBox">
            <input
              type="password"
              required
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />{" "}
            <i>Password</i>
          </div>
          <span className="h-3 mt-[-10px] mb-[5px] text-sm text-red-300">
            {errorMessage}
          </span>
          <div className="links mb-[-7px]">
            <Link draggable="false" className="a" href="#">
              Forgot Password
            </Link>
            <Link draggable="false" className="a" href="/signin">
              Signup
            </Link>
          </div>

          <Button
            className="input-button input text-lg hover:bg-zinc-200 bg-zinc-100 text-black font-bold"
            color="white"
            onClick={handleLogin}
          >
            Login
          </Button>
        </div>
      </div>
    </div>
  );
}
