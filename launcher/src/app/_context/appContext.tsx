"use client";

import { useEffect, useState, createContext, ReactNode, FC } from "react";
import useNetworkStatus from "../useNetworkStatus";
import NetworkOffline from "../networkOffline";
import { invoke } from "@tauri-apps/api/tauri";
import { Minus, X } from "lucide-react";
import { usePathname } from "next/navigation";

interface AppState {
  appStarted: boolean;
}

interface AppContextProps {
  appState: AppState;
  setAppState: React.Dispatch<React.SetStateAction<AppState>>;
}

const AppContext = createContext<AppContextProps | undefined>(undefined);

const AppProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [appState, setAppState] = useState<AppState>({
    appStarted: false,
  });
  const networkChecker = useNetworkStatus();

  const [started, setStarted] = useState(false);
  const path = usePathname();
  useEffect(() => {
    document.body.style.overflow = "hidden";
    //document.addEventListener("contextmenu", (event) => event.preventDefault());
    /*
    const f = (e: any) => {
      for (let i = 1; i <= 12; i++) {
        if (e.key == `F${i}`) {
          e.preventDefault();
        }
      }
      if (
        (e.ctrlKey || e.shiftKey) &&
        !(e.ctrlKey && ["C", "c", "A", "a", "V", "v"].includes(e.key))
      ) {
        e.preventDefault();
      }
    };
    
    document.onkeydown = f;
    document.onclick = f;
    */
    if (!started && networkChecker.isOnline) {
      setTimeout(() => setAppState({ appStarted: true }), 5700);
      invoke("not_connected");
      setStarted(true);
    }
  }, [appState.appStarted, started, setAppState, networkChecker.isOnline]);
  return (
    <AppContext.Provider value={{ appState, setAppState }}>
      {path == "/home/goplay" ? (
        children
      ) : (
        <main
          className="flex flex-col items-center bg-black h-screen justify-center w-full"
          data-tauri-drag-region
        >
          <span
            data-tauri-drag-region
            className="text-white opacity-90 absolute top-3 left-4 z-10  w-fit selectDisable"
          >
            Survive the troll
          </span>
          <div className="flex items-center justify-between gap-3 absolute top-3 right-3">
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
          {networkChecker.isOnline && typeof window !== "undefined" ? (
            children
          ) : (
            <NetworkOffline data-tauri-drag-region />
          )}
        </main>
      )}
    </AppContext.Provider>
  );
};

export { AppContext, AppProvider };
