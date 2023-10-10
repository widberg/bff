import React, { useState } from "react";
import {
  BigFileData,
  ResourcePreview,
  ViewTab, ResourceInfo,
  Sort,
} from "../types/types";

import "./Explorer.css";
import { updateView } from "../functions/preview";

function ResourceButton({
  bffObjectName = "",
  implemented = true,
  name = 0,
  // onClick,
  setResourcePreview,
  setOpenTab,
}: {
  bffObjectName: string;
  implemented: boolean;
  name: number;
  // onClick: any;
  setResourcePreview: React.Dispatch<React.SetStateAction<ResourcePreview | null>>;
  setOpenTab: React.Dispatch<React.SetStateAction<ViewTab>>;
}) {
  return (
    <button
      className={`bffobject ${implemented ? "" : "bffobject-unimpl"}`}
      onClick={() => {
        updateView(name, setResourcePreview, setOpenTab);
      }}
    >
      {bffObjectName}
    </button>
  );
}

function ObjectList({
  resources,
  // onClick,
  sort,
  sortBackward,
  setResourcePreview,
  setOpenTab,
}: {
  resources: ResourceInfo[];
  // onClick: any;
  sort: number;
  sortBackward: boolean;
  setResourcePreview: React.Dispatch<React.SetStateAction<ResourcePreview | null>>;
  setOpenTab: React.Dispatch<React.SetStateAction<ViewTab>>;
}) {
  let objectsCopy = [...resources];
  if (sort != Sort.Block) objectsCopy.sort((a, b) => a.name - b.name);
  if (sort == Sort.Extension)
    objectsCopy.sort((a, b) => {
      if (a.class_name !== null) {
        if (b.class_name !== null)
          return (a.class_name as string).localeCompare(
            b.class_name as string
          );
        else return -1;
      } else if (b.class_name !== null) return 1;
      else return 0;
    });
  if (sortBackward) objectsCopy.reverse();

  let btns: JSX.Element[] = objectsCopy.map((v: ResourceInfo, i: number) => (
    <ResourceButton
      key={i}
      // implemented={v.is_implemented}
      implemented={true}
      bffObjectName={`${v.name}.${
        v.class_name ? v.class_name : "unimplemented"
      }`}
      name={v.name}
      // onClick={onClick}
      setResourcePreview={setResourcePreview}
      setOpenTab={setOpenTab}
    />
  ));
  return <div>{btns}</div>;
}

function SortButton({
  onClick,
  id,
  name,
  sort,
  sortBackward,
}: {
  onClick: any;
  id: number;
  name: string;
  sort: number;
  sortBackward: boolean;
}) {
  return (
    <button onClick={() => onClick(id)}>
      <span>{name}</span>
      {sort == id && (
        <span className="explorer-sort-arrow">{sortBackward ? "▲" : "▼"}</span>
      )}
    </button>
  );
}

export function Explorer({
  bigfile,
  setResourcePreview,
  setOpenTab,
}: {
  bigfile: BigFileData;
  setResourcePreview: React.Dispatch<React.SetStateAction<ResourcePreview | null>>;
  setOpenTab: React.Dispatch<React.SetStateAction<ViewTab>>;
}) {
  const [sort, setSort] = useState<Sort>(Sort.Block);
  const [sortBackward, setSortBackward] = useState<boolean>(false);

  function sortButtonPress(type: number) {
    setSort(type);
    setSortBackward(sort != type ? false : !sortBackward);
  }

  return (
    <div className="explorer">
      <span className="explorer-header">
        {bigfile.filename !== "" ? bigfile.filename : "BigFile structure"}
      </span>
      <span className="explorer-sort second-header">
        <SortButton
          onClick={sortButtonPress}
          id={Sort.Block}
          name="Default"
          sort={sort}
          sortBackward={sortBackward}
        />
        <SortButton
          onClick={sortButtonPress}
          id={Sort.Name}
          name="Name"
          sort={sort}
          sortBackward={sortBackward}
        />
        <SortButton
          onClick={sortButtonPress}
          id={Sort.Extension}
          name="Extension"
          sort={sort}
          sortBackward={sortBackward}
        />
      </span>
      <div className="bffobject-list">
        <ObjectList
          resources={bigfile.resource_infos}
          // onClick={updatePreview}
          sort={sort}
          sortBackward={sortBackward}
          setResourcePreview={setResourcePreview}
          setOpenTab={setOpenTab}
        />
      </div>
    </div>
  );
}
