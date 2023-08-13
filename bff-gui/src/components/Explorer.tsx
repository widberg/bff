import { useState } from "react";
import {
  BFFObject,
  BigFileData,
  PreviewObject,
  PreviewTab,
  Sort,
} from "../types/types";

import "./Explorer.css";
import { updatePreview } from "../functions/preview";

function BFFObjectButton({
  bffObjectName = "",
  implemented = true,
  name = 0,
  onClick,
  setPreviewObject,
  setOpenPreviewTab,
}: {
  bffObjectName: string;
  implemented: boolean;
  name: number;
  onClick: any;
  setPreviewObject: React.Dispatch<React.SetStateAction<PreviewObject | null>>;
  setOpenPreviewTab: React.Dispatch<React.SetStateAction<PreviewTab>>;
}) {
  return (
    <button
      className={`bffobject ${implemented ? "" : "bffobject-unimpl"}`}
      onClick={() => {
        onClick(name, setPreviewObject, setOpenPreviewTab);
      }}
    >
      {bffObjectName}
    </button>
  );
}

function ObjectList({
  bffObjects,
  onClick,
  sort,
  sortBackward,
  setPreviewObject,
  setOpenPreviewTab,
}: {
  bffObjects: BFFObject[];
  onClick: any;
  sort: number;
  sortBackward: boolean;
  setPreviewObject: React.Dispatch<React.SetStateAction<PreviewObject | null>>;
  setOpenPreviewTab: React.Dispatch<React.SetStateAction<PreviewTab>>;
}) {
  let objectsCopy = [...bffObjects];
  if (sort != Sort.Block) objectsCopy.sort((a, b) => a.name - b.name);
  if (sort == Sort.Extension)
    objectsCopy.sort((a, b) => {
      if (a.real_class_name !== null) {
        if (b.real_class_name !== null)
          return (a.real_class_name as string).localeCompare(
            b.real_class_name as string
          );
        else return -1;
      } else if (b.real_class_name !== null) return 1;
      else return 0;
    });
  if (sortBackward) objectsCopy.reverse();

  let btns: JSX.Element[] = objectsCopy.map((v: BFFObject, i: number) => (
    <BFFObjectButton
      key={i}
      implemented={v.is_implemented}
      bffObjectName={`${v.name}.${
        v.real_class_name ? v.real_class_name : "unimplemented"
      }`}
      name={v.name}
      onClick={onClick}
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
  setPreviewObject: React.Dispatch<React.SetStateAction<PreviewObject | null>>;
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
        {bigfile.name !== "" ? bigfile.name : "BigFile structure"}
      </span>
      <span className="explorer-sort second-header">
        <SortButton
          onClick={sortButtonPress}
          id={Sort.Block}
          name="Block"
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
          bffObjects={bigfile.objects}
          onClick={updatePreview}
          sort={sort}
          sortBackward={sortBackward}
          setPreviewObject={setPreviewObject}
          setOpenPreviewTab={setOpenPreviewTab}
        />
      </div>
    </div>
  );
}
