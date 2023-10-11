import React, { useState } from "react";
import {
  BigFileData,
  ResourcePreview,
  ViewTab,
  ResourceInfo,
  Sort,
  Nickname,
} from "../types/types";

import "./Explorer.css";
import { updateView } from "../functions/preview";
import { VariableSizeList as List } from "react-window";
import AutoSizer from "react-virtualized-auto-sizer";

function ResourceButton({
  // implemented = true,
  name = 0,
  classN = "",
  nickname = "",
  selected = false,
  // onClick,
  setResourcePreview,
  setOpenTab,
  setCurrentNickname,
  style,
}: {
  // implemented: boolean;
  name: number;
  classN: string;
  nickname: string;
  selected: boolean;
  // onClick: any;
  setResourcePreview: React.Dispatch<
    React.SetStateAction<ResourcePreview | null>
  >;
  setOpenTab: React.Dispatch<React.SetStateAction<ViewTab>>;
  setCurrentNickname: React.Dispatch<React.SetStateAction<string>>;
  style: any;
}) {
  return (
    <button
      // className={`bffobject ${implemented ? "" : "bffobject-unimpl"}`}
      className={`resource ${selected ? "resource-open" : ""}`}
      onClick={() => {
        updateView(name, setResourcePreview, setOpenTab);
        setCurrentNickname(nickname);
      }}
      style={style}
    >
      <span>
        {nickname !== "" ? `${nickname}.${classN}` : `${name}.${classN}`}
      </span>
      {nickname !== "" && <span className="resource-realname">({name})</span>}
    </button>
  );
}

function ResourceList({
  resources,
  nicknames,
  sort,
  sortBackward,
  setResourcePreview,
  setOpenTab,
  setCurrentNickname,
}: {
  resources: ResourceInfo[];
  nicknames: Nickname[];
  sort: number;
  sortBackward: boolean;
  setResourcePreview: React.Dispatch<
    React.SetStateAction<ResourcePreview | null>
  >;
  setOpenTab: React.Dispatch<React.SetStateAction<ViewTab>>;
  setCurrentNickname: React.Dispatch<React.SetStateAction<string>>;
}) {
  let objectsCopy = [...resources];
  if (sort != Sort.Default) objectsCopy.sort((a, b) => a.name - b.name);
  if (sort == Sort.Extension)
    objectsCopy.sort((a, b) => {
      if (a.class_name !== null) {
        if (b.class_name !== null)
          return a.class_name.localeCompare(b.class_name);
        else return -1;
      } else if (b.class_name !== null) return 1;
      else return 0;
    });
  if (sortBackward) objectsCopy.reverse();

  const Row = ({ index, style }: { index: any; style: any }) => (
    <ResourceButton
      key={index}
      name={objectsCopy[index].name}
      classN={objectsCopy[index].class_name}
      nickname={
        nicknames.find((nickname: Nickname) => {
          return nickname.name === objectsCopy[index].name;
        })?.nickname ?? ""
      }
      selected={false}
      // onClick={onClick}
      setResourcePreview={setResourcePreview}
      setOpenTab={setOpenTab}
      setCurrentNickname={setCurrentNickname}
      style={{
        ...style,
        top: `${parseFloat(style.top) + 25}px`,
      }}
    />
  );
  return (
    <AutoSizer>
      {({ height, width }: { height: number; width: number }) => (
        <List
          width={width}
          height={height}
          itemCount={objectsCopy.length}
          itemSize={() => 25}
        >
          {Row}
        </List>
      )}
    </AutoSizer>
  );
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
  nicknames,
  setResourcePreview,
  setOpenTab,
  setCurrentNickname,
}: {
  bigfile: BigFileData;
  nicknames: Nickname[];
  setResourcePreview: React.Dispatch<
    React.SetStateAction<ResourcePreview | null>
  >;
  setOpenTab: React.Dispatch<React.SetStateAction<ViewTab>>;
  setCurrentNickname: React.Dispatch<React.SetStateAction<string>>;
}) {
  const [sort, setSort] = useState<Sort>(Sort.Default);
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
          id={Sort.Default}
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
      <div className="resource-list">
        <ResourceList
          resources={bigfile.resource_infos}
          nicknames={nicknames}
          // onClick={updatePreview}
          sort={sort}
          sortBackward={sortBackward}
          setResourcePreview={setResourcePreview}
          setOpenTab={setOpenTab}
          setCurrentNickname={setCurrentNickname}
        />
      </div>
    </div>
  );
}
