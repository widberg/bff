import { useComponentVisible } from "../hooks/click-outside";
import {
  BigFileData,
  DataType,
  Nickname,
  ResourcePreview,
} from "../types/types";

import "./Menubar.css";

import {
  exportAll,
  exportNicknames,
  exportOne,
  exportPreview,
} from "../functions/export";
import { selectBigfile } from "../functions/bigfile";

function ExportMenu({
  bigfileLoaded,
  bigfileName,
  resourcePreview,
  nicknames,
}: {
  bigfileLoaded: boolean;
  bigfileName: string;
  resourcePreview: ResourcePreview | null;
  nicknames: Nickname[];
}) {
  const { ref, isComponentVisible, setIsComponentVisible } =
    useComponentVisible(false);
  return (
    <div ref={ref}>
      <button
        onClick={() => {
          setIsComponentVisible(!isComponentVisible);
        }}
      >
        Export
      </button>
      <div
        className="submenu"
        style={{
          display: isComponentVisible ? "flex" : "none",
        }}
      >
        <button
          onClick={() => {
            setIsComponentVisible(false);
            exportAll();
          }}
          disabled={!bigfileLoaded}
        >
          Export all resources as JSON...
        </button>
        <button
          onClick={() => {
            setIsComponentVisible(false);
            exportOne(resourcePreview?.name ?? 0);
          }}
          disabled={resourcePreview === null}
        >
          Export resource as JSON...
        </button>
        <button
          onClick={() => {
            setIsComponentVisible(false);
            exportPreview(
              resourcePreview?.name ?? 0,
              resourcePreview?.preview_data?.data_type as DataType
            );
          }}
          disabled={
            resourcePreview === null || resourcePreview?.preview_data === null
          }
        >
          Export preview...
        </button>
        <button
          onClick={() => {
            setIsComponentVisible(false);
            exportNicknames(bigfileName, nicknames);
          }}
          disabled={nicknames.length == 0}
        >
          Export nicknames...
        </button>
      </div>
    </div>
  );
}

export function Menubar({
  bigfileLoaded,
  bigfileName,
  resourcePreview,
  nicknames,
  setResourcePreview,
  setBigfile,
}: {
  bigfileLoaded: boolean;
  bigfileName: string;
  resourcePreview: ResourcePreview | null;
  nicknames: Nickname[];
  setResourcePreview: React.Dispatch<
    React.SetStateAction<ResourcePreview | null>
  >;
  setBigfile: React.Dispatch<React.SetStateAction<BigFileData>>;
}) {
  return (
    <div className="menubar">
      <button onClick={() => selectBigfile(setResourcePreview, setBigfile)}>
        Open BigFile...
      </button>
      <ExportMenu
        bigfileLoaded={bigfileLoaded}
        bigfileName={bigfileName}
        resourcePreview={resourcePreview}
        nicknames={nicknames}
      />
    </div>
  );
}
