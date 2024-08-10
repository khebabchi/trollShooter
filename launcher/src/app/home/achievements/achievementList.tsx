import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
} from "@/components/ui/pagination";
import Achievement, { achievement } from "./achievement";

import { useEffect, useState } from "react";

export default function AchievementsList({
  finalAchs,
}: {
  finalAchs: achievement[];
}) {
  // start pagination
  const [pgInd, setPgInd] = useState(1);
   useEffect(() => {
    const cmp = finalAchs.length / 4 + (finalAchs.length % 4 != 0 ? 1 : 0);
     if (pgInd>cmp) setPgInd(cmp);
   }, [finalAchs.length,pgInd]);
  let paginationItems: React.JSX.Element[] = [];
  for (
    let i = 1;
    i <= finalAchs.length / 4 + (finalAchs.length % 4 != 0 ? 1 : 0);
    i++
  ) {
    paginationItems.push(
      <PaginationItem>
        <PaginationLink onClick={() => setPgInd(i)} isActive={i == pgInd}>
          {i}
        </PaginationLink>
      </PaginationItem>
    );
  }
  if (paginationItems.length == 0)
    paginationItems.push(
      <PaginationItem>
        <PaginationLink isActive>1</PaginationLink>
      </PaginationItem>
    );
  // end pagination

  let displayedChs: React.JSX.Element[] = [];
  for (const ach of finalAchs.slice(
    (pgInd - 1) * 4,
    Math.min(finalAchs.length, 4 + (pgInd - 1) * 4)
  )) {
    displayedChs.push(<Achievement data={ach} />);
  }

  return (
    <>
      <div
        className="flex flex-col h-[340px] gap-4 items-center"
        data-tauri-drag-region
      >
        {displayedChs}
      </div>
      <Pagination>
        <PaginationContent>{paginationItems}</PaginationContent>
      </Pagination>
    </>
  );
}
