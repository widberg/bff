import { useState } from "react";
import {
  BigFileData,
  ResourcePreview,
  PreviewTab, ResourceInfo,
  Sort,
} from "../types/types";

import "./Explorer.css";
import { updatePreview } from "../functions/preview";

function ResourceButton({
  bffObjectName = "",
  implemented = true,
  name = 0,
  // onClick,
  setPreviewObject,
  setOpenPreviewTab,
}: {
  bffObjectName: string;
  implemented: boolean;
  name: number;
  // onClick: any;
  setPreviewObject: React.Dispatch<React.SetStateAction<ResourcePreview | null>>;
  setOpenPreviewTab: React.Dispatch<React.SetStateAction<PreviewTab>>;
}) {
  return (
    <button
      className={`bffobject ${implemented ? "" : "bffobject-unimpl"}`}
      onClick={() => {
        updatePreview(name, setPreviewObject, setOpenPreviewTab);
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
  setPreviewObject,
  setOpenPreviewTab,
}: {
  resources: ResourceInfo[];
  // onClick: any;
  sort: number;
  sortBackward: boolean;
  setPreviewObject: React.Dispatch<React.SetStateAction<ResourcePreview | null>>;
  setOpenPreviewTab: React.Dispatch<React.SetStateAction<PreviewTab>>;
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
      setPreviewObject={setPreviewObject}
      setOpenPreviewTab={setOpenPreviewTab}
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
  setPreviewObject,
  setOpenPreviewTab,
}: {
  bigfile: BigFileData;
  setPreviewObject: React.Dispatch<React.SetStateAction<ResourcePreview | null>>;
  setOpenPreviewTab: React.Dispatch<React.SetStateAction<PreviewTab>>;
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
          setPreviewObject={setPreviewObject}
          setOpenPreviewTab={setOpenPreviewTab}
        />
      </div>
    </div>
  );
}
