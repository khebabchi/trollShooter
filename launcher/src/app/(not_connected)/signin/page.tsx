"use client";
import "../style.css";
import Link from "next/link";
import Logo from "../../logo";
import { useContext, useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { useRouter } from "next/navigation";
import { AppContext, User } from "@/app/_context/appContext";
const emailRegex =
  /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/;
export default function Signin() {
  if (typeof window !== "undefined") {
    window.__TAURI__.tauri.invoke("signin");
  }

  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
  async function handleSignIn() {
    setErrorMessage("");
    const url = `https://trollshooterbackend-production.up.railway.app/user/signin`;
    try {
      const response = await window.__TAURI__.http.fetch(url, {
        method: "POST",
        headers: {
          "Content-Type": "application/json", // Correctly set the content type
        },
        body: window.__TAURI__.http.Body.json({
          username,
          email,
          password,
          topScore: 0,
          createdAt: "",
        }),
      });
      if (!response.ok) {
        setErrorMessage("User exists");
      } else {
        const user: User = response.data;
        if(!user){
          setErrorMessage("Server error");
        }{
           window.__TAURI__.tauri.invoke("set_user", { user });
        }
       
      }
    } catch (error: any) {
      setErrorMessage("Username exists, try to login instead");
      console.error(error);
    }
  }
  const router = useRouter();
  const context = useContext(AppContext);
  const appStarted = context?.appState.appStarted;
  useEffect(() => {
    function handleKeyPress(event: KeyboardEvent) {
      if (event.key === "Enter") {
        handleSignIn();
      }
    }

    window.addEventListener("keydown", handleKeyPress);

    return () => {
      window.removeEventListener("keydown", handleKeyPress);
    };
  });
  return (
    <div className="signin" data-tauri-drag-region>
      <div className="content" data-tauri-drag-region>
        <Logo appStarted={appStarted} />
        <h2>Sign In</h2>

        <div className="form" data-tauri-drag-region>
          <div className="inputBox">
            <input
              type="text"
              required
              value={email}
              onChange={(e) => setEmail(e.target.value)}
            />
            <i>Email</i>
          </div>

          <div className="inputBox">
            <input
              type="email"
              required
              value={username}
              onChange={(e) => setUsername(e.target.value)}
            />
            <i>Username</i>
          </div>
          <div className="inputBox">
            <input
              type="password"
              required
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
            <i>Password</i>
          </div>
          <div className="inputBox">
            <input
              type="password"
              required
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
            />
            <i>Confirm password</i>
          </div>
          <span className="h-3 mt-[-10px] mb-[5px] text-sm text-red-300">
            {errorMessage}
          </span>
          <div className="links  mb-[-7px]">
            <div className=" flex ">
              <span className="a hover:bg-black opacity-60 w-full h-auto mr-2">
                Already have an account ?
              </span>
              <Link className="a" href="/login">
                Login
              </Link>
            </div>
          </div>

          <Button
            className="input-button input text-lg hover:bg-zinc-200 bg-zinc-100 text-black font-bold"
            color="white"
            onClick={() => {
              setErrorMessage("");
              if (!emailRegex.test(email)) {
                console.log(username.length);
                setErrorMessage("Incorrect email");
              } else if (username.length <= 3) {
                setErrorMessage("Username must be at least 4 characters long.");
              } else if (password.length <= 4) {
                setErrorMessage("Password must be at least 5 characters long.");
              } else if (!/[a-zA-Z]/.test(password)) {
                setErrorMessage("Password should contain letters a-z A-Z");
              } else if (!/\d/.test(password)) {
                setErrorMessage("Password should contain numbers.");
              } else if (password != confirmPassword) {
                setErrorMessage("Passwords do not match");
              } else {
                handleSignIn();
              }
            }}
          >
            Signin
          </Button>
        </div>
      </div>
    </div>
  );
}
