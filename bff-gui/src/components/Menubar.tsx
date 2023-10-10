import { useComponentVisible } from "../hooks/click-outside";
import { BigFileData, DataType, ResourcePreview } from "../types/types";

import "./Menubar.css";

import { exportAll, exportOne, exportPreview } from "../functions/export";
import { selectBigfile } from "../functions/bigfile";

function ExportMenu({
  bigfileLoaded,
  resourcePreview,
}: {
  bigfileLoaded: boolean;
  resourcePreview: ResourcePreview | null;
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
            exportOne(resourcePreview?.name as number);
          }}
          disabled={resourcePreview === null}
        >
          Export resource as JSON...
        </button>
        <button
          onClick={() => {
            setIsComponentVisible(false);
            exportPreview(
              resourcePreview?.name as number,
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

export function Menubar({
  bigfileLoaded,
  resourcePreview,
  setResourcePreview,
  setBigfile,
}: {
  bigfileLoaded: boolean;
  resourcePreview: ResourcePreview | null;
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
        resourcePreview={resourcePreview}
      />
    </div>
  );
}
