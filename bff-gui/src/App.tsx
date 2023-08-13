import { useState } from "react";

import { Preview } from "./components/Preview";
import { Explorer } from "./components/Explorer";
import { Menubar } from "./components/Menubar";

import { BigFileData, PreviewObject, PreviewTab } from "./types/types";

function App() {
  const [bigfile, setBigfile] = useState<BigFileData>({
    name: "",
    objects: [],
  });
  const [previewObject, setPreviewObject] = useState<PreviewObject | null>(
    null
  );
  const [openPreviewTab, setOpenPreviewTab] = useState<PreviewTab>(
    PreviewTab.Preview
  );

  return (
    <div className="container">
      <Menubar
        bigfileLoaded={bigfile.name !== ""}
        previewObject={previewObject}
        setPreviewObject={setPreviewObject}
        setBigfile={setBigfile}
      />
      <div className="main">
        <Explorer
          bigfile={bigfile}
          setPreviewObject={setPreviewObject}
          setOpenPreviewTab={setOpenPreviewTab}
        />
        <Preview
          previewObject={previewObject}
          openPreviewTab={openPreviewTab}
          setOpenPreviewTab={setOpenPreviewTab}
        />
      </div>
    </div>
  );
}

export default App;
