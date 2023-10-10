import { useComponentVisible } from "../hooks/click-outside";
import { BigFileData, ResourcePreview } from "../types/types";

import "./Menubar.css";

import { exportAll, exportOne, exportPreview } from "../functions/export";
import { selectBigfile } from "../functions/bigfile";

function ExportMenu({
  bigfileLoaded,
  previewObject,
}: {
  bigfileLoaded: boolean;
  previewObject: ResourcePreview | null;
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
          Export objects as JSON...
        </button>
        <button
          onClick={() => {
            setIsComponentVisible(false);
            exportOne(previewObject?.name as number);
          }}
          disabled={previewObject === null}
        >
          Export current object as JSON...
        </button>
        <button
          onClick={() => {
            setIsComponentVisible(false);
            exportPreview(
              previewObject?.name as number,
              previewObject?.preview_path as string
            );
          }}
          disabled={
            previewObject === null || previewObject?.preview_path === null
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
  setResourcePreview: React.Dispatch<React.SetStateAction<ResourcePreview | null>>;
  setBigfile: React.Dispatch<React.SetStateAction<BigFileData>>;
}) {
  return (
    <div className="menubar">
      <button onClick={() => selectBigfile(setResourcePreview, setBigfile)}>
        Open BigFile...
      </button>
      <ExportMenu bigfileLoaded={bigfileLoaded} previewObject={resourcePreview} />
    </div>
  );
}
