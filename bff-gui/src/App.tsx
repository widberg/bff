import { useState } from "react";

import { View } from "./components/View";
import { Explorer } from "./components/Explorer";
import { Menubar } from "./components/Menubar";

import { BigFileData, ResourcePreview, ViewTab } from "./types/types";

function App() {
  const [bigfile, setBigfile] = useState<BigFileData>({
    filename: "",
    resource_infos: [],
  });
  const [resourcePreview, setResourcePreview] = useState<ResourcePreview | null>(
    null
  );
  const [openTab, setOpenTab] = useState<ViewTab>(
    ViewTab.Preview
  );

  return (
    <div className="container">
      <Menubar
        bigfileLoaded={bigfile.filename !== ""}
        resourcePreview={resourcePreview}
        setResourcePreview={setResourcePreview}
        setBigfile={setBigfile}
      />
      <div className="main">
        <Explorer
          bigfile={bigfile}
          setResourcePreview={setResourcePreview}
          setOpenTab={setOpenTab}
        />
        <View
          resourcePreview={resourcePreview}
          openTab={openTab}
          setOpenTab={setOpenTab}
        />
      </div>
    </div>
  );
}

export default App;
