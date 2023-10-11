import { useComponentVisible } from "../hooks/click-outside";
import {
  BigFileData,
  DataType,
  Nickname,
  ResourcePreview,
} from "../types/types";

import "./Menubar.css";

import { exportAll, exportOne, exportPreview } from "../functions/export";
import { selectBigfile } from "../functions/bigfile";
import {
  clearNicknames,
  exportNicknames,
  importNicknames,
} from "../functions/nicknames";

function ExportMenu({
  bigfileLoaded,
  resourcePreview,
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
      </div>
    </div>
  );
}

function NicknameMenu({
  bigfileLoaded,
  bigfileName,
  nicknames,
  setNicknames,
}: {
  bigfileLoaded: boolean;
  bigfileName: string;
  nicknames: Nickname[];
  setNicknames: React.Dispatch<React.SetStateAction<Nickname[]>>;
}) {
  const { ref, isComponentVisible, setIsComponentVisible } =
    useComponentVisible(false);
  return (
    <div ref={ref}>
      <button
        onClick={() => {
          setIsComponentVisible(!isComponentVisible);
        }}
        disabled={!bigfileLoaded}
      >
        Nicknames
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
            importNicknames(setNicknames);
          }}
        >
          Import nicknames...
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
        <button
          onClick={() => {
            setIsComponentVisible(false);
            clearNicknames(setNicknames);
          }}
          disabled={nicknames.length == 0}
        >
          Clear nicknames
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
  setNicknames,
}: {
  bigfileLoaded: boolean;
  bigfileName: string;
  resourcePreview: ResourcePreview | null;
  nicknames: Nickname[];
  setResourcePreview: React.Dispatch<
    React.SetStateAction<ResourcePreview | null>
  >;
  setBigfile: React.Dispatch<React.SetStateAction<BigFileData>>;
  setNicknames: React.Dispatch<React.SetStateAction<Nickname[]>>;
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
      <NicknameMenu
        bigfileLoaded={bigfileLoaded}
        bigfileName={bigfileName}
        nicknames={nicknames}
        setNicknames={setNicknames}
      />
    </div>
  );
}
