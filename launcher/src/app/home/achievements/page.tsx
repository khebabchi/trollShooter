"use client";
import AchievementsList from "./achievementList";
import { Filter } from "lucide-react";
import achievements from "@/../public/achievements.js";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { Checkbox } from "@/components/ui/checkbox";
import { useEffect, useState } from "react";
import { CheckedState } from "@radix-ui/react-checkbox";
import { achievement } from "./achievement";
import { User } from "@/app/_context/appContext";
import { invoke } from "@tauri-apps/api/tauri";
import { fetch } from "@tauri-apps/api/http";
type FetchType = { id: number; username: string; unlocked_at: string }[];
export default function AchievementsLayout() {
  const [open, setOpen] = useState(false);
  const [filters, setFilters] = useState(new Set([1, 2, 3, 4, 5]));
  const [search, setSearch] = useState("");
  const [fetched, setFetched] = useState<FetchType>([]);
  function HandleSelectChange(checked: CheckedState, i: number) {
    const newFilters = new Set(filters); // Create a copy of the current filters
    if (checked) {
      newFilters.add(i); // Add the filter if it's checked
    } else {
      newFilters.delete(i); // Remove the filter if it's unchecked
    }
    setFilters(newFilters);
  }

  //------------------------------------------------------------------------------------------------------------
  async function fetchAchievements(username: string | undefined) {
    try {
      const response = await fetch(
        `https://trollshooterbackend-production.up.railway.app/users/${username}/achievements`
      );
      if (response.ok) {
        setFetched(response.data as FetchType);
        console.log(response.data);
      }
    } catch (error: any) {
      console.error(error);
    }
  }
  //------------------------------------------------------------------------------------------------------------
  function setupAchievements(
    fetched: { id: number; username: string; unlocked_at: string }[]
  ): achievement[] {
    const achs = achievements.map((data, ind) => {
      return {
        done: fetched.some((ach) => {
          return ach.id === ind + 1;
        }),
        ...data,
      };
    });
    const sortedAchs = achs.sort((ach_1, ach_2) => {
      if (ach_1.done === ach_2.done) {
        return ach_2.rarity - ach_1.rarity;
      }
      return ach_1.done ? -1 : 1;
    });
    const FiltersedAch: achievement[] = [];
    sortedAchs.forEach((ach) => {
      filters.has(ach.rarity) &&
        ((ach.done && filters.has(4)) || (!ach.done && filters.has(5))) &&
        FiltersedAch.push(JSON.parse(JSON.stringify(ach)));
    });
    const SearchAch: achievement[] = [];
    FiltersedAch.forEach((ach) => {
      (ach.title.toLowerCase().includes(search.toLowerCase()) ||
        ach.description.toLowerCase().includes(search.toLowerCase())) &&
        SearchAch.push(JSON.parse(JSON.stringify(ach)));
    });
    return SearchAch;
  }
  useEffect(() => {
    invoke("get_user").then((user) => {
      fetchAchievements((user as User).username);
    });
  }, [setFetched]);
  //------------------------------------------------------------------------------------------------------------

  return (
    <div className="flex flex-col gap-5 items-center" data-tauri-drag-region>
      <div className="flex gap-2">
        <input
          value={search}
          onChange={(e) => {
            setSearch(e.target.value);
          }}
          className="bg-zinc-900 border-[1px] border-zinc-700 rounded-xl focus:border-zinc-500  focus:outline-none px-3 py-1 placeholder:font-medium w-[250px]"
          placeholder="Search"
        />
        <Popover open={open} onOpenChange={(newOpen) => setOpen(newOpen)}>
          <PopoverTrigger
            className={`${
              open ? "bg-zinc-100" : "bg-zinc-400"
            } rounded-xl  hover:bg-zinc-100  focus:outline-none px-[10px] py-1 placeholder:font-medium  cursor-pointer`}
          >
            <Filter size={17} color="black" />
          </PopoverTrigger>
          <PopoverContent className="gap-3 text-white flex flex-col pt-3 bg-zinc-900 border-zinc-700 ml-[155px] w-[200px]">
            Filters
            <ul className="list-disc list-inside text-sm flex flex-col gap-2 text-zinc-400">
              <li>
                <span className="ml-[-5px]">Rarity :</span>
                <ul className="ml-[20px] my-1">
                  <li className="text-[#f5d36e] flex items-center">
                    <Checkbox
                      checked={filters.has(3)}
                      onCheckedChange={(checked) =>
                        HandleSelectChange(checked, 3)
                      }
                      className="border-zinc-300 w-[14px] h-[14px] mr-2"
                    />
                    Legandary
                  </li>
                  <li className="text-[#c550ff] flex items-center">
                    <Checkbox
                      checked={filters.has(2)}
                      onCheckedChange={(checked) =>
                        HandleSelectChange(checked, 2)
                      }
                      className="border-zinc-300 w-[14px] h-[14px] mr-2"
                    />
                    Rare
                  </li>
                  <li className="text-zinc-200  flex items-center">
                    <Checkbox
                      checked={filters.has(1)}
                      onCheckedChange={(checked) =>
                        HandleSelectChange(checked, 1)
                      }
                      className="border-zinc-300 w-[14px] h-[14px] mr-2"
                    />
                    Commun
                  </li>
                </ul>
              </li>
              <li>
                <span className="ml-[-5px] ">Completion :</span>
                <ul className="ml-[20px] my-1">
                  <li className="text-green-100 flex items-center">
                    <Checkbox
                      checked={filters.has(4)}
                      onCheckedChange={(checked) =>
                        HandleSelectChange(checked, 4)
                      }
                      className="border-zinc-300 w-[14px] h-[14px] mr-2"
                    />
                    Completed
                  </li>
                  <li className="text-red-100 flex items-center">
                    <Checkbox
                      checked={filters.has(5)}
                      onCheckedChange={(checked) =>
                        HandleSelectChange(checked, 5)
                      }
                      className="border-zinc-300 w-[14px] h-[14px] mr-2"
                    />
                    Locked
                  </li>
                </ul>
              </li>
            </ul>
          </PopoverContent>
        </Popover>
      </div>

      <AchievementsList finalAchs={setupAchievements(fetched)} />
    </div>
  );
}
